use crate::executor::Executor;
use rdkafka::message::OwnedMessage;

pub(crate) struct DummyExecutor {}

impl Executor for DummyExecutor {
    fn execute(&self, _message: &OwnedMessage) -> anyhow::Result<()> {
        info!("I'm doing nothing :/");

        Ok(())
    }
}
