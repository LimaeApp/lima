use crate::serve::LimaServerState;
use axum::Router;
use axum::routing::{get, post};

pub mod apps_router {
    use super::*;

    pub fn create() -> Router<LimaServerState> {
        Router::new()
            .route("/blocked", get(get_all_blocked_apps))
            .route("/block", post(block_app))
            .route("/unblock", post(unblock_app))
    }

    async fn get_all_blocked_apps() {}
    async fn block_app() {}
    async fn unblock_app() {}
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
