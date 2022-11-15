use anyhow::anyhow;
use reqwest::Url;
use rocket::serde::json::Json;
use rocket::State;
use rocket_validation::Validated;

use util::models::manager::{ApiCreateLoadTaskRequest, ApiCreateLoadTaskResponse};
use util::models::{ApiErrorMessage, ApiErrors, ApiOkMessage, ApiResult};
use crate::processor::Processor;

#[get("/hello/world")]
pub(crate) async fn hello_world(_p: &State<Processor>) -> ApiResult<Json<ApiOkMessage>> {
    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}

#[post("/task/create", data = "<request>")]
pub(crate) async fn create_load_task(
    request: Validated<Json<ApiCreateLoadTaskRequest>>,
    p: &State<Processor>,
) -> ApiResult<Json<ApiCreateLoadTaskResponse>> {
    for endpoint in request.0.endpoints.iter() {
        match Url::parse(endpoint) {
            Ok(_) => (),
            Err(err) => {
                return Err(ApiErrorMessage {
                    code: ApiErrors::UrlParseError as i8,
                    message: ApiErrors::wrap_error(err),
                });
            }
        }
    }

    let mut tx = match p.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    let count = match p.manager.check_status(&mut tx, "pending").await {
        Ok(count) => count,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    if count > 0 {
        return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_anyhow_error(anyhow!("pending task currently present")),
        });
    }

    let task = match p.manager.create_task(&mut tx, &request.0).await {
        Ok(task) => task,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    match p.manager.spawn_jobs(&mut tx, &task).await {
        Ok(_) => (),
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    tx.commit().await;

    Ok(Json(
        ApiCreateLoadTaskResponse {
            uuid: task.uuid.to_string(),
            endpoints: task.endpoints,
            iterations: task.iterations,
            meta: task.meta,
        }
    ))
}
