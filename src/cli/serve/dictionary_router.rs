use crate::serve::LimaServerState;
use crate::utils::{ResultToJson, run_if_valid_bearer};
use crate::utils::{SerializeResultToJson, transaction_execute};
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum_auth::AuthBearer;
use serde_json::Value;

use crate::models::Dictionary;
use axum::routing::delete;
use sqlx::Pool;

pub fn create() -> Router<LimaServerState> {
    Router::new()
        .route(
            "/",
            get(get_all_from_dict)
                .post(add_to_dict)
                .delete(delete_all_from_dict),
        )
        .route("/item", delete(delete_from_dict))
}

async fn get_all_from_dict(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        SerializeResultToJson::to_json_result(
            sqlx::query_as::<_, Dictionary>("SELECT * FROM Dictionary;")
                .fetch_all(&state.db_pool)
                .await,
        )
    })
    .await
}

async fn add_to_dict(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(payload): Json<Vec<Dictionary>>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        let mut bulk_insert_query = String::from("INSERT INTO Dictionary (id, string) VALUES ");

        payload.iter().enumerate().for_each(|(iteration, _)| {
            if iteration > 0 {
                bulk_insert_query.push_str(", ");
            }
            let base = iteration * 2;
            bulk_insert_query.push_str(&format!("(${}, ${})", base + 1, base + 2));
        });

        let mut transaction = Pool::begin(&state.db_pool).await.map_err(|e| {
            eprintln!("Failed to begin transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut query_builder = sqlx::query(&bulk_insert_query);
        let payload_len = payload.len();

        for dictionary in payload {
            query_builder = query_builder.bind(dictionary.id).bind(dictionary.string);
        }

        query_builder
            .execute(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Couldn't add strings to dictionary: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        transaction.commit().await.map_err(|e| {
            eprintln!("[Commit failed] Couldn't add strings to dictionary: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(Value::String(format!(
            "Added {} strings to dictionary",
            payload_len
        ))))
    })
    .await
}

async fn delete_all_from_dict(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        transaction_execute(
            &(state.db_pool),
            sqlx::query("DELETE FROM Dictionary;"),
            "Couldn't delete all strings from dictionary",
            "Couldn't delete all strings from dictionary",
            "Removed all strings from dictionary!",
        )
        .await
    })
    .await
}

async fn delete_from_dict(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(payload): Json<Dictionary>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        sqlx::query("DELETE FROM Dictionary WHERE id = ? AND string = ?")
            .bind(payload.id)
            .bind(&payload.string)
            .execute(&state.db_pool)
            .await
            .to_json_result(format!("Removed {} from dictionary!", payload.string).to_string())
    })
    .await
}
