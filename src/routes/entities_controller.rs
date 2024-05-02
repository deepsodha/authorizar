use crate::dto::entities::{Entity, EntityInput};
use crate::routes::api_error::ApiError;
use crate::routes::api_response::ApiResponse;
use crate::routes::app_state::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;
use sqlx::{self, SqlitePool};

#[post("")]
pub async fn add(
    app_state: web::Data<AppState>,
    entity_input: web::Json<EntityInput>,
) -> Result<ApiResponse> {
    let mut tr = app_state.pool.begin().await?;

    let current_time = Utc::now();

    let insert_query = "INSERT INTO entities 
        (id,eid, etype, content, search_tags, created_ts, updated_ts)
         VALUES($1,$2,$3,$4,$5,$6,$7)  RETURNING id ";

    let result: Result<String, sqlx::Error> = sqlx::query_scalar(insert_query)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(entity_input.uid.id.clone())
        .bind(entity_input.uid.r#type.clone())
        .bind(serde_json::to_value(entity_input)?)
        .bind("".to_string())
        .bind(current_time.to_rfc3339())
        .bind("".to_string())
        .fetch_one(&mut tr)
        .await;

    let id = match result {
        Ok(id) => id,
        Err(err) => {
            tr.rollback().await?; // Rollback the transaction in case of an error
            return Err(Error::from(err).into()); // Convert sqlx::Error to your custom error type
        }
    };

    tr.commit().await?;

    Ok(ApiResponse {
        status_code: StatusCode::OK,
        message: "Successfully added entity",
        data: Some(id),
    })
}


#[get("")]
pub async fn get_all(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    match get_all_entities(&app_state.pool).await {
        Ok(entities) => {
            let entities_value: serde_json::Value = serde_json::to_value(entities)?;

            Ok(HttpResponse::Ok().json(ApiResponse {
                status_code: "200".to_string(),
                message: "Successful".to_string(),
                data: entities_value,
            }))
        }
        Err(e) => Err(ApiError::NotFound(e.to_string())),
    }
}

async fn get_all_entities(pool: &SqlitePool) -> Result<Vec<Entity>, sqlx::Error> {
    let entities = sqlx::query_as::<sqlx::Sqlite, Entity>(
        "SELECT id,eid,etype,content,search_tags,created_ts,updated_ts FROM entities",
    )
    .fetch_all(pool)
    .await?;
    Ok(entities)
}

#[get("/{id}")]
pub async fn get_by_id(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let entity_id = path.into_inner(); // Extract the ID from the path

    match get_entity_by_id(&app_state.pool, entity_id).await {
        Ok(entities) => {
            let entities_value: serde_json::Value = serde_json::to_value(entities)?;

            Ok(HttpResponse::Ok().json(ApiResponse {
                status_code: "200".to_string(),
                message: "Successful".to_string(),
                data: entities_value,
            }))
        }
        Err(e) => Err(ApiError::NotFound(e.to_string())),
    }
}

async fn get_entity_by_id(pool: &SqlitePool, id: String) -> Result<Entity, sqlx::Error> {
    let query =
        "SELECT id,eid,etype,content,search_tags,created_ts,updated_ts FROM entities WHERE id = $1";
    let rows = sqlx::query_as::<_, Entity>(query)
        .bind(&id)
        .fetch_one(pool)
        .await?;

    Ok(rows)
}

#[put("/{id}")]
pub async fn update(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
    entity_input: web::Json<EntityInput>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let mut tr = app_state.pool.begin().await?;

    let current_time = Utc::now();

    let query = "UPDATE entities SET etype = $1, content = $2, search_tags = $3,
                        updated_ts =$4 , eid=$5 WHERE id = $6 RETURNING id";

    let row: (String,) = sqlx::query_as(query)
        .bind(entity_input.uid.r#type.clone())
        .bind(serde_json::to_value(entity_input.clone())?)
        .bind("".to_string())
        .bind(current_time.to_rfc3339())
        .bind(entity_input.uid.id.clone())
        .bind(id)
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
    let entity_id = path.into_inner();
    let mut tr = app_state.pool.begin().await?;

    let query = "
    DELETE FROM entities
    WHERE id = $1 RETURNING id;
";

    let row: (String,) = sqlx::query_as(query)
        .bind(entity_id)
        .fetch_one(&mut tr)
        .await?;
    let deleted_id = row.0;
    tr.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status_code: "200".to_string(),
        message: "Successful".to_string(),
        data: serde_json::Value::String(deleted_id),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add)
        .service(update)
        .service(remove)
        .service(get_all)
        .service(get_by_id);
}
