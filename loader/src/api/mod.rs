use std::collections::HashMap;
use rocket::serde::json::Json;
use rocket::State;
use rocket_validation::Validated;

use util::models::loader::{ApiAccountingStatistics, ApiGetUsersByProviderRequest,
                           ApiGetUsersByProviderResponse, ApiProviderCreateRequest,
                           ApiUserAccumulated, ApiUserCreateRequest, ApiUserDeleteRequest};
use util::models::{ApiErrorMessage, ApiErrors, ApiOkMessage, ApiResult};
use crate::cmd::Repository;

#[post("/hello/world")]
pub(crate) async fn hello_world(_r: &State<Repository>) -> ApiResult<Json<ApiOkMessage>> {
    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}

#[post("/users/create", data = "<request>")]
pub(crate) async fn create_user(
    request: Validated<Json<ApiUserCreateRequest>>,
    r: &State<Repository>,
) -> ApiResult<Json<ApiOkMessage>> {
    let mut tx = match r.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    let provider = match r.accounting.get_provider_by_affiliation(&mut tx, &request.0.company_name).await {
        Ok(provider) => provider,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    let user = match r.accounting.create_user(&mut tx, &request.0, provider.id).await {
        Ok(user) => user,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    match r.accounting.generate_accounting(&mut tx, user.id).await {
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

    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}

#[post("/providers/create", data = "<request>")]
pub(crate) async fn create_provider(
    request: Validated<Json<ApiProviderCreateRequest>>,
    r: &State<Repository>,
) -> ApiResult<Json<ApiOkMessage>> {
    let mut tx = match r.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    match r.accounting.create_provider(&mut tx, &request.0).await {
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

    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}

#[post("/users/get_by_provider", data = "<request>")]
pub(crate) async fn get_users_by_provider(
    request: Validated<Json<ApiGetUsersByProviderRequest>>,
    r: &State<Repository>,
) -> ApiResult<Json<ApiGetUsersByProviderResponse>> {
    let mut tx = match r.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    let users = match r.accounting.filter_users_by_provider(&mut tx, &request.0.provider).await {
        Ok(users) => users,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    let mut users_accumulated = Vec::new();

    for user in users.iter() {
        let accounting = match r.accounting.get_user_accounting(&mut tx, user.id).await {
            Ok(accounting) => accounting,
            Err(err) => {
                tx.rollback().await;

                return Err(ApiErrorMessage {
                    code: ApiErrors::DatabaseError as i8,
                    message: ApiErrors::wrap_anyhow_error(err),
                });
            }
        };

        let mut counts = HashMap::new();
        let mut payments = Vec::new();
        let mut taxes = Vec::new();

        for acc in accounting.iter() {
            match counts.get_mut(&acc.payment_type) {
                Some(counter) => *counter += 1,
                None => match counts.insert(acc.payment_type.clone(), 1) {
                    Some(_) => (),
                    None => ()
                }
            }

            payments.push(acc.amount);
            taxes.push(acc.tax_charged);
        }

        users_accumulated.push(
            ApiUserAccumulated {
                name: user.name.clone(),
                surname: user.surname.clone(),
                birth_date: user.birth_date.clone(),
                company_name: user.company_name.clone(),
                provider_name: user.provider_name.clone(),
                accounting_statistics: ApiAccountingStatistics {
                    counts,
                    payments: HashMap::from([
                        ("paid_total".to_string(), payments.iter().sum::<i32>() as f32),
                        ("paid_mean".to_string(), payments.iter().sum::<i32>() as f32 / payments.len() as f32),
                        ("tax_total".to_string(), taxes.iter().sum::<i32>() as f32),
                        ("tax_mean".to_string(), taxes.iter().sum::<i32>() as f32 / taxes.len() as f32),
                    ]),
                },
            }
        )
    }

    tx.commit().await;

    Ok(Json(
        ApiGetUsersByProviderResponse {
            data: users_accumulated
        }
    ))
}

#[post("/users/delete", data = "<request>")]
pub(crate) async fn delete_user(
    request: Validated<Json<ApiUserDeleteRequest>>,
    r: &State<Repository>,
) -> ApiResult<Json<ApiOkMessage>> {
    let mut tx = match r.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Err(ApiErrorMessage {
            code: ApiErrors::DatabaseError as i8,
            message: ApiErrors::wrap_error(err),
        })
    };

    let user = match r.accounting.search_for_user(&mut tx, &request.0.name, &request.0.surname).await {
        Ok(user) => user,
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    match r.accounting.delete_user_accounting(&mut tx, user.id).await {
        Ok(_) => (),
        Err(err) => {
            tx.rollback().await;

            return Err(ApiErrorMessage {
                code: ApiErrors::DatabaseError as i8,
                message: ApiErrors::wrap_anyhow_error(err),
            });
        }
    };

    match r.accounting.delete_user(&mut tx, user.id).await {
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

    Ok(Json(ApiOkMessage {
        result: "ok".to_owned()
    }))
}
