use crate::serve::LimaServerState;
use axum::Router;
use axum::routing::get;

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
