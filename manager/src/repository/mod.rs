pub(crate) mod models;

use sqlx::{Postgres, Transaction};
use util::models::manager::ApiCreateLoadTaskRequest;
use crate::repository::models::{QueueItem, Task};

#[async_trait]
pub(crate) trait Manager {
    async fn create_task(&self, tx: &mut Transaction<Postgres>, task: &ApiCreateLoadTaskRequest) -> anyhow::Result<Task>;
    async fn check_status(&self, tx: &mut Transaction<Postgres>, status: &str) -> anyhow::Result<i32>;
    async fn spawn_jobs(&self, tx: &mut Transaction<Postgres>, task: &Task) -> anyhow::Result<()>;
    async fn get_next_job(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<Option<QueueItem>>;
    async fn apply_job(&self, tx: &mut Transaction<Postgres>, job_id: i32, response: f64) -> anyhow::Result<()>;
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

    async fn check_status(&self, tx: &mut Transaction<Postgres>, status: &str) -> anyhow::Result<i32> {
        let count = sqlx::query!(
            r#"
            select count(1) as "count!" from task
            where status = $1
            "#,
            status
        )
            .fetch_one(tx)
            .await?;

        Ok(count.count.try_into()?)
    }

    async fn spawn_jobs(&self, tx: &mut Transaction<Postgres>, task: &Task) -> anyhow::Result<()> {
        for _ in 0..task.iterations {
            for endpoint in task.endpoints.iter() {
                let job = sqlx::query!(
                r#"
                insert into queue (task_id, endpoint)
                values ($1, $2)
                "#,
                task.id, endpoint
            )
                    .execute(&mut *tx)
                    .await?;
            }
        }

        Ok(())
    }

    async fn get_next_job(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<Option<QueueItem>> {
        let job = sqlx::query_as!(
            QueueItem,
            r#"
            with next_job as (
                select id from queue
                where status = 0
                limit 1
                for update skip locked
            )
            update queue
            set
                status = 1
            from next_job
            where queue.id = next_job.id
            returning queue.id, task_id, endpoint, response, status
            "#
        )
            .fetch_optional(tx)
            .await?;

        Ok(job)
    }

    async fn apply_job(&self, tx: &mut Transaction<Postgres>, job_id: i32, response: f64) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            update queue
            set
                status = 2,
                response = $2
            where id = $1
            "#,
            job_id, response
        )
            .execute(tx)
            .await?;

        Ok(())
    }
}
