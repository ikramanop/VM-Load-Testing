use std::collections::HashMap;
use anyhow::anyhow;
use reqwest::Url;
use rocket::serde::json::Json;
use rocket::State;
use rocket_validation::Validated;

use util::models::manager::{ApiCreateLoadTaskRequest, ApiCreateLoadTaskResponse, ApiGetTaskSummaryRequest, ApiGetTaskSummaryResponse, STATUS_DONE};
use util::models::{ApiErrorMessage, ApiErrors, ApiOkMessage, ApiResult};
use crate::processor::Processor;
use crate::repository::models::Statistic;

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

#[post("/task/summary", data = "<request>")]
pub(crate) async fn get_task_summary(
    request: Validated<Json<ApiGetTaskSummaryRequest>>,
    p: &State<Processor>,
) -> ApiResult<Json<ApiGetTaskSummaryResponse>> {
    let mut tx = match p.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    let task = match p.manager.get_task(&mut tx, &request.0.uuid).await {
        Ok(task) => task,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    let statistics: Statistic;

    if task.status == STATUS_DONE {
        statistics = match p.manager.get_statistics(&mut tx, task.id).await {
            Ok(statistics) => statistics,
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        };
    } else {
        let queue_count = match p.manager.check_queue(&mut tx, task.id).await {
            Ok(queue_count) => queue_count,
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        };

        if queue_count > 0 {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::QueueError as i8,
                message: ApiErrors::wrap_anyhow_error(anyhow!("queue items still in progress")),
            });
        }

        match p.manager.update_status(&mut tx, task.id, STATUS_DONE).await {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        }

        statistics = match p.manager.calculate_statistics(&mut tx, task.id).await {
            Ok(statistics) => statistics,
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        };
    }

    let mut stats = HashMap::<String, f64>::new();

    stats.insert("avg".to_string(), statistics.avg);
    stats.insert("std".to_string(), statistics.std);
    stats.insert("min".to_string(), statistics.min);
    stats.insert("max".to_string(), statistics.max);

    match request.0.base_avg {
        Some(base_avg) => {
            let queue = match p.manager.get_queue(&mut tx, task.id).await {
                Ok(queue) => queue,
                Err(err) => {
                    tx.rollback().await;

                    return Err(ApiErrorMessage {
                        code: ApiErrors::DatabaseError as i8,
                        message: ApiErrors::wrap_anyhow_error(err),
                    });
                }
            };

            if queue.len() == 0 {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::QueueError as i8,
                    message: ApiErrors::wrap_anyhow_error(anyhow!("empty queue for this task")),
                });
            }

            let mut num = 0f64;
            for q in queue.iter() {
                num += q.response.unwrap() - base_avg;
            }

            stats.insert("shifted_std".to_string(), (num * num / queue.len() as f64).sqrt());
        }
        None => ()
    }

    if request.0.three_sigma_percent {
        let queue = match p.manager.get_queue(&mut tx, task.id).await {
            Ok(queue) => queue,
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        };

        let mut count = 0f64;
        for q in queue.iter() {
            if q.response.unwrap() > (statistics.avg + 3f64 * statistics.std) {
                count += 1f64
            }
        }

        stats.insert("three_percentile_percent".to_string(), count / queue.len() as f64 * 100f64);
    }

    tx.commit().await;

    Ok(Json(ApiGetTaskSummaryResponse {
        uuid: task.uuid,
        endpoints: task.endpoints,
        iterations: task.iterations,
        meta: task.meta,
        statistics: stats,
    }))
}
