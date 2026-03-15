use crate::serve::LimaServerState;
use crate::utils::SerializeResultToJson;
use crate::utils::{ResultToJson, run_if_valid_bearer};
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_auth::AuthBearer;
use serde_json::Value;

use crate::models::{BlockApp, BlockedApp};

pub fn create() -> Router<LimaServerState> {
    Router::new()
        .route("/blocked", get(get_all_blocked_apps))
        .route("/block", post(block_app))
        .route("/unblock", post(unblock_app))
}

async fn get_all_blocked_apps(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        SerializeResultToJson::to_json_result(
            sqlx::query_as::<_, BlockedApp>("SELECT * FROM AppBlocklist")
                .fetch_all(&state.db_pool)
                .await,
        )
    })
    .await
}

async fn block_app(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(payload): Json<BlockApp>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        let package_name = payload.package_name;
        sqlx::query("INSERT INTO AppBlocklist (id, package_name) VALUES (?, ?)")
            .bind(payload.id)
            .bind(&package_name)
            .execute(&state.db_pool)
            .await
            .to_json_result(format!("Added {} to blocklist!", package_name).to_string())
    })
    .await
}

async fn unblock_app(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(payload): Json<BlockApp>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        let package_name = payload.package_name;
        sqlx::query("DELETE FROM AppBlocklist WHERE id = ? AND package_name = ?")
            .bind(payload.id)
            .bind(&package_name)
            .execute(&state.db_pool)
            .await
            .to_json_result(format!("Removed {} from blocklist!", package_name).to_string())
    })
    .await
}
