use crate::AppConfig;
use clap::Parser;
use sqlx::{PgPool, Pool, Postgres};
use crate::repository::{Accounting, AccountingRepository};
use crate::api::{create_user, hello_world, create_provider, get_users_by_provider, delete_user};

#[derive(Parser)]
#[clap(name = "loader")]
#[clap(bin_name = "loader")]
pub(crate) enum CMD {
    Api,
    Migrate,
}

pub(crate) struct Repository {
    pub(crate) pool: Pool<Postgres>,
    pub(crate) accounting: Box<dyn Accounting + Sync + Send>,
}

pub(crate) async fn run_api_cmd(config: &AppConfig) -> anyhow::Result<()> {
    let pool = PgPool::connect(&config.database_url).await?;

    let launch_result =
        rocket::build()
            .mount(
                "/",
                routes![
                    create_user,
                    hello_world,
                    create_provider,
                    get_users_by_provider,
                    delete_user
                ],
            )
            .manage(Repository {
                pool,
                accounting: Box::new(AccountingRepository {}),
            })
            .launch()
            .await;

    match launch_result {
        Ok(_) => info!("Rocket shut down gracefully"),
        Err(err) => error!("Rocket shut down with an error: {}", err),
    };

    Ok(())
}

pub(crate) async fn run_migrate_cmd(config: &AppConfig) -> anyhow::Result<()> {
    let pool = PgPool::connect(&config.database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    info!("Migrations up done");

    Ok(())
}