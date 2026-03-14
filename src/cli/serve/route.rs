use crate::serve::LimaServerState;
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};

pub mod apps_router {
    use super::*;
    use crate::models::{BlockApp, BlockedApp};
    use crate::utils::SerializeResultToJson;
    use crate::utils::{ResultToJson, run_if_valid_bearer};
    use axum_auth::AuthBearer;
    use serde_json::Value;

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
                .fetch_all(&state.db_pool)
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
                .fetch_all(&state.db_pool)
                .await
                .to_json_result(format!("Removed {} from blocklist!", package_name).to_string())
        })
        .await
    }
}

pub mod notes_router {
    use super::*;

    pub fn create() -> Router<LimaServerState> {
        Router::new()
            .route(
                "/",
                get(get_all_notes)
                    .post(insert_note)
                    .delete(delete_all_notes),
            )
            .route(
                "/{id}",
                get(get_a_note).put(update_a_note).delete(delete_a_note),
            )
    }

    async fn get_all_notes() {}
    async fn insert_note() {}
    async fn delete_all_notes() {}
    async fn get_a_note() -> &'static str {
        "noice"
    }
    async fn update_a_note() {}
    async fn delete_a_note() {}
}

pub mod dictionary_router {
    use super::*;
    use axum::routing::delete;

    pub fn create() -> Router<LimaServerState> {
        Router::new()
            .route(
                "/",
                get(get_all_from_dict)
                    .post(add_to_dict)
                    .delete(delete_all_from_dict),
            )
            .route("/item/{id}", delete(delete_from_dict))
    }

    async fn get_all_from_dict() {}
    async fn add_to_dict() {}
    async fn delete_all_from_dict() {}
    async fn delete_from_dict() {}
}
