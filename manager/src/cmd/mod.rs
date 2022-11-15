use std::thread::sleep;
use std::time::Duration;
use crate::AppConfig;
use clap::Parser;
use sqlx::PgPool;
use util::clients::{LoaderClient, LoaderClientStub};
use crate::repository::ManagerRepository;
use crate::api::{hello_world, create_load_task};
use crate::processor::Processor;

#[derive(Parser)]
#[clap(name = "manager")]
#[clap(bin_name = "manager")]
pub(crate) enum CMD {
    Manager,
    Generator,
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
            .manage(Processor {
                pool,
                manager: Box::new(ManagerRepository {}),
                loader: Box::new(LoaderClientStub {}),
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
    let r = Processor {
        pool: PgPool::connect(&config.database_url).await?,
        manager: Box::new(ManagerRepository {}),
        loader: Box::new(LoaderClient {}),
    };

    loop {
        match r.process_queue().await {
            Ok(result) => match result {
                Some(_) => println!("Message successfully handled"),
                None => {
                    if config.poll == 0 {
                        println!("No jobs in queue. Exiting....");
                        break;
                    }
                    println!("Got no new jobs. Sleeping....");
                    sleep(Duration::from_secs(1));
                }
            }
            Err(err) => {
                println!("Got got an error: {}. Recovering....", err);
                sleep(Duration::from_secs(1));
            }
        }
    }

    Ok(())
}
