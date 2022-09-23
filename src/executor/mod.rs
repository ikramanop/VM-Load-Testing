use crate::config::WorkerConfig;
use crate::Config;
use anyhow::anyhow;
use async_trait::async_trait;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::OwnedMessage;
use rdkafka::ClientConfig;

pub(crate) mod dummy;

pub(crate) trait Executor: Sync {
    fn execute(&self, message: &OwnedMessage) -> anyhow::Result<()>;
}

#[async_trait]
pub(crate) trait Engine {
    fn new(
        global_config: &Config,
        worker_config: Box<dyn WorkerConfig>,
        executor: Box<dyn Executor>,
    ) -> Self;

    async fn perform_testing(&self) -> anyhow::Result<()>;
}

pub(crate) struct LoadTestingEngine {
    global_config: Config,
    worker_config: Box<dyn WorkerConfig>,
    executor: Box<dyn Executor>,
}

#[async_trait]
impl Engine for LoadTestingEngine {
    fn new(
        global_config: &Config,
        worker_config: Box<dyn WorkerConfig>,
        executor: Box<dyn Executor>,
    ) -> Self {
        LoadTestingEngine {
            global_config: global_config.clone(),
            worker_config,
            executor,
        }
    }

    async fn perform_testing(&self) -> anyhow::Result<()> {
        let consumer: &StreamConsumer = &ClientConfig::new()
            .set("group.id", &self.worker_config.kafka_group_id())
            .set("bootstrap.servers", &self.global_config.kafka_host)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "10000")
            .set("enable.auto.commit", "false")
            .create()?;

        consumer.subscribe(&[&self.worker_config.kafka_topic()])?;

        loop {
            match consumer.recv().await {
                Ok(borrowed_message) => {
                    self.executor.execute(&borrowed_message.detach())?;

                    consumer
                        .commit_message(&borrowed_message, CommitMode::Async)
                        .unwrap();
                }
                Err(err) => return Err(anyhow!("{:?}", err)),
            }
        }
    }
}
