use crate::dto::policies::{Policy, PolicyInput};
use crate::routes::api_error::ApiError;
use crate::routes::api_response::ApiResponse;
use crate::routes::app_state::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;
use sqlx::SqlitePool;

#[post("")]
pub async fn add(
    app_state: web::Data<AppState>,
    policy_input: web::Json<PolicyInput>,
) -> Result<HttpResponse, ApiError> {
    let mut tr = app_state.pool.begin().await?;

    let current_time = Utc::now();

    let insert_query = "INSERT INTO policies 
        (id, ttl, content, search_tags, created_ts, updated_ts)
         VALUES($1,$2,$3,$4,$5,$6) RETURNING id";

    let row: (String,) = sqlx::query_as(insert_query)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(1)
        .bind(&policy_input.content)
        .bind(serde_json::Value::Object(Default::default()))
        .bind(current_time.to_rfc3339())
        .bind("".to_string())
        .fetch_one(&mut tr)
        .await?;

    let id = row.0;
    tr.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status_code: "200".to_string(),
        message: "Successful".to_string(),
        data: serde_json::Value::String(id),
    }))
}

#[get("")]
pub async fn get_all(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    match get_all_policies(&app_state.pool).await {
        Ok(policies) => {
            let policies_value: serde_json::Value = serde_json::to_value(policies)?;

            Ok(HttpResponse::Ok().json(ApiResponse {
                status_code: "200".to_string(),
                message: "Successful".to_string(),
                data: policies_value,
            }))
        }
        Err(e) => Err(ApiError::NotFound(e.to_string())),
    }
}

async fn get_all_policies(pool: &SqlitePool) -> Result<Vec<Policy>, sqlx::Error> {
    let policies = sqlx::query_as::<sqlx::Sqlite, Policy>(
        "SELECT id,ttl,content,search_tags,created_ts,updated_ts FROM policies",
    )
    .fetch_all(pool)
    .await?;
    Ok(policies)
}

#[put("/{id}")]
pub async fn update(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
    policy_input: web::Json<PolicyInput>,
) -> Result<HttpResponse, ApiError> {
    let policy_id = path.into_inner();

    let mut tr = app_state.pool.begin().await?;

    let current_time = Utc::now();

    let query = "UPDATE policies SET ttl = $1, content = $2, search_tags = $3,
                        created_ts = $4 , updated_ts =$5 WHERE id = $6 RETURNING id";
    let row: (String,) = sqlx::query_as(query)
        .bind(2)
        .bind(&policy_input.content)
        .bind(serde_json::Value::Object(Default::default()))
        .bind("".to_string())
        .bind(current_time.to_rfc3339())
        .bind(policy_id)
        .fetch_one(&mut tr)
        .await?;
    let updated_id = row.0;
    tr.commit().await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        status_code: "200".to_string(),
        message: "Successful".to_string(),
        data: serde_json::Value::String(updated_id),
    }))
}

#[delete("/{id}")]
pub async fn remove(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let policy_id = path.into_inner();
    let mut tr = app_state.pool.begin().await?;
    let query = "
    DELETE FROM policies
    WHERE id = $1 RETURNING id;
";

    let row: (String,) = sqlx::query_as(query)
        .bind(policy_id)
        .fetch_one(&mut tr)
        .await?;
    let deleted_id = row.0; //
    tr.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status_code: "200".to_string(),
        message: "Successful".to_string(),
        data: serde_json::Value::String(deleted_id),
    }))
}

#[get("/{id}")]
pub async fn get_by_id(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let policy_id = path.into_inner();

    match get_policy_by_id(&app_state.pool, policy_id).await {
        Ok(policies) => {
            let policies_value: serde_json::Value = serde_json::to_value(policies)?;

            Ok(HttpResponse::Ok().json(ApiResponse {
                status_code: "200".to_string(),
                message: "Successful".to_string(),
                data: policies_value,
            }))
        }
        Err(e) => Err(ApiError::NotFound(e.to_string())),
    }
}

async fn get_policy_by_id(pool: &SqlitePool, id: String) -> Result<Policy, sqlx::Error> {
    let policy = sqlx::query_as::<sqlx::Sqlite, Policy>(
        "SELECT id,ttl,content,search_tags,created_ts,updated_ts FROM policies WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(policy)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add)
        .service(update)
        .service(remove)
        .service(get_all)
        .service(get_by_id);
}
