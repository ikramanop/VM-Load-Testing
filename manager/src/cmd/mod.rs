use std::collections::HashMap;
use crate::AppConfig;
use clap::Parser;
use sqlx::{PgPool, Pool, Postgres};
use crate::repository::{Manager, ManagerRepository};
use crate::api::{hello_world, create_load_task};

#[derive(Parser)]
#[clap(name = "manager")]
#[clap(bin_name = "manager")]
pub(crate) enum CMD {
    Manager,
    Generator,
}

pub(crate) struct Repository {
    pub(crate) pool: Pool<Postgres>,
    pub(crate) manager: Box<dyn Manager + Sync + Send>,
}

pub(crate) async fn run_manager_cmd(config: &AppConfig) -> anyhow::Result<()> {
    let pool = PgPool::connect(&config.database_url).await?;

    let launch_result =
        rocket::build()
            .mount(
                "/",
                routes![
                    hello_world,
                    create_load_task
                ],
            )
            .manage(Repository {
                pool,
                manager: Box::new(ManagerRepository {}),
            })
            .launch()
            .await;

    match launch_result {
        Ok(_) => info!("Rocket shut down gracefully"),
        Err(err) => error!("Rocket shut down with an error: {}", err),
    };

    Ok(())
}

pub(crate) async fn run_generator_cmd(config: &AppConfig) -> anyhow::Result<()> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}
