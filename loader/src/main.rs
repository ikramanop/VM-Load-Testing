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

use crate::cmd::{run_api_cmd, CMD, run_migrate_cmd};
use crate::config::AppConfig;
use envconfig::Envconfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = AppConfig::init_from_env().unwrap();

    let binary = CMD::parse();

    match binary {
        CMD::Api => Ok(run_api_cmd(&config).await?),
        CMD::Migrate => Ok(run_migrate_cmd(&config).await?)
    }
}
