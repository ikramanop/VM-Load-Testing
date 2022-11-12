#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod cmd;
mod config;
mod repository;

#[macro_use]
extern crate rocket;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;

use crate::cmd::{CMD, run_manager_cmd, run_generator_cmd};
use crate::config::AppConfig;
use envconfig::Envconfig;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = AppConfig::init_from_env().unwrap();

    let binary = CMD::parse();

    match binary {
        CMD::Manager => Ok(run_manager_cmd(&config).await?),
        CMD::Generator => Ok(run_generator_cmd(&config).await?)
    }
}
