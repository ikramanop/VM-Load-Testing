use std::collections::HashMap;
use std::time::Instant;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use fake::Fake;
use fake::faker::name::raw::FirstName;
use fake::faker::name::raw::LastName;
use fake::locales::EN;
use sqlx::{Pool, Postgres};
use util::clients::Loader;
use util::models::loader::{ApiGetUsersByProviderRequest, ApiUserCreateRequest, ApiUserDeleteRequest};
use crate::repository::Manager;

pub(crate) struct Processor {
    pub(crate) pool: Pool<Postgres>,
    pub(crate) manager: Box<dyn Manager + Sync + Send>,
    pub(crate) loader: Box<dyn Loader + Sync + Send>,
}

impl Processor {
    pub(crate) async fn process_queue(&self) -> anyhow::Result<Option<()>> {
        let mut tx = self.pool.begin().await?;

        let job = match self.manager.get_next_job(&mut tx).await? {
            Some(job) => job,
            None => {
                tx.rollback().await?;
                return Ok(None);
            }
        };

        let name: String = FirstName(EN).fake();
        let surname: String = LastName(EN).fake();

        println!("Trying job with id {}", job.id);

        let now = Instant::now();

        match self.loader.create_user(&job.endpoint, ApiUserCreateRequest {
            name: name.clone(),
            surname: name.clone(),
            birth_date: NaiveDateTime::parse_from_str("2000-06-15T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            company_name: "LLC \"RNG TECH\"".to_string(),
        }).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(anyhow!(err));
            }
        };

        match self.loader.get_users_by_provider(&job.endpoint, ApiGetUsersByProviderRequest {
            provider: "ank".to_string()
        }).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(anyhow!(err));
            }
        };

        match self.loader.delete_user(&job.endpoint, ApiUserDeleteRequest {
            name: name[1..].to_string(),
            surname: name[1..].to_string(),
        }).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(anyhow!(err));
            }
        };

        match self.loader.get_users_by_provider(&job.endpoint, ApiGetUsersByProviderRequest {
            provider: "ank".to_string()
        }).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(anyhow!(err));
            }
        };

        let elapsed = now.elapsed();

        println!("Response: {:?}", elapsed);

        match self.manager.apply_job(&mut tx, job.id, elapsed.as_secs_f64()).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };

        tx.commit().await?;

        Ok(Some(()))
    }
}