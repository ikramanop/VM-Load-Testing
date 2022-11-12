use rocket::serde::json::Json;
use rocket::State;
use rocket_validation::Validated;

use crate::api::models::{ApiCreateLoadTaskRequest, ApiCreateLoadTaskResponse, ApiErrorMessage, ApiErrors, ApiOkMessage};
use crate::cmd::Repository;

pub(crate) mod models;

type ApiResult<T> = Result<T, Json<ApiErrorMessage>>;

#[get("/hello/world")]
pub(crate) async fn hello_world(_r: &State<Repository>) -> ApiResult<Json<ApiOkMessage>> {
    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}

#[post("/task/create", data = "<request>")]
pub(crate) async fn create_load_task(
    request: Validated<Json<ApiCreateLoadTaskRequest>>,
    r: &State<Repository>,
) -> ApiResult<Json<ApiCreateLoadTaskResponse>> {
    let mut tx = match r.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(Json(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        }))
    };

    let task = match r.manager.create_task(&mut tx, &request.0).await {
        Ok(task) => task,
        Err(err) => {
            tx.rollback().await;

            return Err(Json(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            }));
        }
    };

    match r.manager.spawn_jobs(&mut tx, &task).await {
        Ok(_) => (),
        Err(err) => {
            tx.rollback().await;

            return Err(Json(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            }));
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
