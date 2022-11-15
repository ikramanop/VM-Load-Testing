pub(crate) mod models;

use chrono::{Duration, Utc};
use rand::{SeedableRng};
use sqlx::{Postgres, Transaction};
use crate::repository::models::{Provider, User, UserExt, Accounting as AccountingModel};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use util::models::loader::{ApiProviderCreateRequest, ApiUserCreateRequest};

const AMOUNTS: &[i32] = &[100000, 150000, 200000];
const TYPES: &[&str] = &["advance", "salary", "vacation"];
const TAXES: &[i32] = &[13000, 19500, 26000];

#[async_trait]
pub(crate) trait Accounting {
    async fn get_provider_by_affiliation(&self, tx: &mut Transaction<Postgres>, company_name: &str) -> anyhow::Result<Provider>;
    async fn create_user(&self, tx: &mut Transaction<Postgres>, user: &ApiUserCreateRequest, affiliation_id: i32) -> anyhow::Result<User>;
    async fn create_provider(&self, tx: &mut Transaction<Postgres>, provider: &ApiProviderCreateRequest) -> anyhow::Result<()>;
    async fn generate_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()>;
    async fn filter_users_by_provider(&self, tx: &mut Transaction<Postgres>, provider_filter: &str) -> anyhow::Result<Vec<UserExt>>;
    async fn get_user_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<Vec<AccountingModel>>;
    async fn search_for_user(&self, tx: &mut Transaction<Postgres>, name: &str, surname: &str) -> anyhow::Result<User>;
    async fn delete_user_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()>;
    async fn delete_user(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()>;
}

pub(crate) struct AccountingRepository {}

#[async_trait]
impl Accounting for AccountingRepository {
    async fn get_provider_by_affiliation(&self, tx: &mut Transaction<Postgres>, company_name: &str) -> anyhow::Result<Provider> {
        let partner = sqlx::query_as!(
            Provider,
            r#"
            select p.id, name, user_limit from provider p
            inner join affiliation a on p.id = a.provider_id
            where a.company_name like $1
            "#,
            format!("%{}%", company_name)
        )
            .fetch_one(tx)
            .await?;

        Ok(partner)
    }

    async fn create_user(&self, tx: &mut Transaction<Postgres>, user: &ApiUserCreateRequest, affiliation_id: i32) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            insert into "user" (affiliation_id, name, surname, birth_date)
            values ($1, $2, $3, $4)
            returning id, affiliation_id, name, surname, birth_date
            "#,
            affiliation_id, user.name, user.surname, user.birth_date
        )
            .fetch_one(tx)
            .await?;

        Ok(user)
    }

    async fn create_provider(&self, tx: &mut Transaction<Postgres>, provider: &ApiProviderCreateRequest) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            insert into "provider" (name, user_limit)
            values ($1, $2)
            "#,
            provider.name, provider.user_limit
        )
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn generate_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()> {
        let now = Utc::now();
        let mut date = Utc::now() - Duration::days(1460);

        let mut rng = StdRng::seed_from_u64(98024749780235712);

        while date < now {
            let amount = AMOUNTS.choose(&mut rng).unwrap();
            let tipo = TYPES.choose(&mut rng).unwrap();
            let tax = TAXES.choose(&mut rng).unwrap();

            sqlx::query!(
            r#"
            insert into "accounting" (user_id, amount, payment_type, tax_charged, date_paid)
            values ($1, $2, $3, $4, $5)
            "#,
                user_id, amount, tipo, tax, date.naive_utc()
            )
                .execute(&mut *tx)
                .await?;

            date = date + Duration::days(5);
        }

        Ok(())
    }

    async fn filter_users_by_provider(&self, tx: &mut Transaction<Postgres>, provider_filter: &str) -> anyhow::Result<Vec<UserExt>> {
        let users = sqlx::query_as!(
            UserExt,
            r#"
            with all_data as (select u.id,
                         u.affiliation_id,
                         u.name,
                         u.surname,
                         u.birth_date,
                         af.company_name,
                         p.name provider_name
                  from "user" u
                           inner join affiliation af on af.id = u.affiliation_id
                           inner join provider p on af.provider_id = p.id)

            select id, affiliation_id, name, surname, birth_date, company_name, provider_name
            from all_data
            where provider_name like $1
            "#,
            format!("%{}%", provider_filter)
        )
            .fetch_all(tx)
            .await?;

        Ok(users)
    }

    async fn get_user_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<Vec<AccountingModel>> {
        let accounting = sqlx::query_as!(
            AccountingModel,
            r#"
            select id, user_id, amount, payment_type, tax_charged, date_paid from accounting
            where user_id = $1
            "#,
            user_id
        )
            .fetch_all(tx)
            .await?;

        Ok(accounting)
    }

    async fn search_for_user(&self, tx: &mut Transaction<Postgres>, name: &str, surname: &str) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id, affiliation_id, name, surname, birth_date from "user"
            where name like $1 and surname like $2
            "#,
            format!("%{}%", name),
            format!("%{}%", surname)
        )
            .fetch_one(tx)
            .await?;

        Ok(user)
    }

    async fn delete_user_accounting(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            delete from accounting
            where user_id = $1
            "#,
            user_id
        )
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn delete_user(&self, tx: &mut Transaction<Postgres>, user_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            delete from "user"
            where id = $1
            "#,
            user_id
        )
            .execute(tx)
            .await?;

        Ok(())
    }
}
