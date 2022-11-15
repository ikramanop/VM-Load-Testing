use anyhow::anyhow;
use reqwest::Url;
use rocket::async_trait;
use rocket::serde::json::serde_json;
use crate::models::ApiErrorMessage;
use crate::models::loader::{ApiGetUsersByProviderRequest, ApiGetUsersByProviderResponse, ApiUserCreateRequest, ApiUserDeleteRequest};

#[async_trait]
pub trait Loader {
    async fn create_user(&self, url: &str, request: ApiUserCreateRequest) -> anyhow::Result<()>;
    async fn delete_user(&self, url: &str, request: ApiUserDeleteRequest) -> anyhow::Result<()>;
    async fn get_users_by_provider(&self, url: &str, request: ApiGetUsersByProviderRequest) -> anyhow::Result<ApiGetUsersByProviderResponse>;
}

pub struct LoaderClient {}

pub struct LoaderClientStub {}

const USERS_CREATE_POSTFIX: &str = "/users/create";
const USERS_DELETE_POSTFIX: &str = "/users/delete";
const USERS_GET_BY_PROVIDER_POSTFIX: &str = "/users/get_by_provider";

#[async_trait]
impl Loader for LoaderClient {
    async fn create_user(&self, url: &str, request: ApiUserCreateRequest) -> anyhow::Result<()> {
        let uri = Url::parse(url)?.join(USERS_CREATE_POSTFIX)?;
        let client = reqwest::Client::new();

        let response = client.post(uri)
            .body(serde_json::to_string(&request)?)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let json = response.json::<ApiErrorMessage>().await?;
                Err(anyhow!(json.message))
            }
            _ => Err(anyhow!(response.status()))
        }
    }

    async fn delete_user(&self, url: &str, request: ApiUserDeleteRequest) -> anyhow::Result<()> {
        let uri = Url::parse(url)?.join(USERS_DELETE_POSTFIX)?;
        let client = reqwest::Client::new();

        let response = client.post(uri)
            .body(serde_json::to_string(&request)?)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let json = response.json::<ApiErrorMessage>().await?;
                Err(anyhow!(json.message))
            }
            _ => Err(anyhow!(response.status()))
        }
    }

    async fn get_users_by_provider(&self, url: &str, request: ApiGetUsersByProviderRequest) -> anyhow::Result<ApiGetUsersByProviderResponse> {
        let uri = Url::parse(url)?.join(USERS_GET_BY_PROVIDER_POSTFIX)?;
        let client = reqwest::Client::new();

        let response = client.post(uri)
            .body(serde_json::to_string(&request)?)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<ApiGetUsersByProviderResponse>().await?),
            reqwest::StatusCode::BAD_REQUEST => {
                let json = response.json::<ApiErrorMessage>().await?;
                Err(anyhow!(json.message))
            }
            _ => Err(anyhow!(response.status()))
        }
    }
}

#[async_trait]
impl Loader for LoaderClientStub {
    async fn create_user(&self, _url: &str, _request: ApiUserCreateRequest) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete_user(&self, _url: &str, _request: ApiUserDeleteRequest) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_users_by_provider(&self, _url: &str, _request: ApiGetUsersByProviderRequest) -> anyhow::Result<ApiGetUsersByProviderResponse> {
        todo!()
    }
}