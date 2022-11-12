mod cmd;
mod config;
mod executor;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use anyhow::anyhow;
use clap::Parser;
use envconfig::Envconfig;

use crate::cmd::CMD;
use crate::config::{Config, WorkerConfig};
use crate::executor::dummy::DummyExecutor;
use crate::executor::{Engine, Executor, LoadTestingEngine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Config::init_from_env()?;

    let bin = CMD::parse();

    let executor: Box<dyn Executor>;
    let worker_config: Box<dyn WorkerConfig>;

    match bin {
        CMD::Disk => return Err(anyhow!("DISK testing unsupported")),

        CMD::Cpu => return Err(anyhow!("CPU testing unsupported")),

        CMD::Ram => return Err(anyhow!("RAM testing unsupported")),

        CMD::Dummy => {
            executor = Box::new(DummyExecutor {});
            worker_config = Box::new(config.dummy.clone());
        }
    }

    LoadTestingEngine::new(&config, worker_config, executor)
        .perform_testing()
        .await
}
