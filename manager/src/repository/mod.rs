pub(crate) mod models;

use sqlx::{Postgres, Transaction};
use crate::repository::models::{Job, Task};
use crate::api::models::ApiCreateLoadTaskRequest;

#[async_trait]
pub(crate) trait Manager {
    async fn create_task(&self, tx: &mut Transaction<Postgres>, task: &ApiCreateLoadTaskRequest) -> anyhow::Result<Task>;
    async fn spawn_jobs(&self, tx: &mut Transaction<Postgres>, task: &Task) -> anyhow::Result<Vec<Job>>;
}

pub(crate) struct ManagerRepository {}

#[async_trait]
impl Manager for ManagerRepository {
    async fn create_task(&self, tx: &mut Transaction<Postgres>, task: &ApiCreateLoadTaskRequest) -> anyhow::Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
            insert into task (endpoints, status, iterations, meta)
            values ($1, $2, $3, $4)
            returning id, uuid::text as "uuid!", endpoints, status, iterations, meta
            "#,
            &task.endpoints, "pending", task.iterations, task.meta
        )
            .fetch_one(tx)
            .await?;

        Ok(task)
    }

    async fn spawn_jobs(&self, tx: &mut Transaction<Postgres>, task: &Task) -> anyhow::Result<Vec<Job>> {
        let mut jobs = Vec::new();

        for endpoint in task.endpoints.iter() {
            let job = sqlx::query_as!(
                Job,
                r#"
                insert into job (task_id, endpoint, status)
                values ($1, $2, $3)
                returning id, task_id, uuid::text as "uuid!", endpoint, status, iteration, updated_at
                "#,
                task.id, endpoint, "pending"
            )
                .fetch_one(&mut *tx)
                .await?;

            jobs.push(job);
        }

        Ok(jobs)
    }
}
