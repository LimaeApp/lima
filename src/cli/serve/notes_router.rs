use crate::models::{IdDTO, Note, Pagination};
use crate::serve::LimaServerState;
use crate::utils::{ResultToJson, SerializeResultToJson, run_if_valid_bearer, transaction_execute};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, put};
use axum::{Json, Router};
use axum_auth::AuthBearer;
use serde_json::Value;

pub fn create() -> Router<LimaServerState> {
    Router::new()
        .route(
            "/",
            get(get_notes).post(insert_note).delete(delete_all_notes),
        )
        .route("/item", put(update_a_note).delete(delete_a_note))
}

async fn get_notes(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        SerializeResultToJson::to_json_result(
            sqlx::query_as::<_, Note>("SELECT * FROM Note WHERE id > ? ORDER BY id ASC LIMIT ?;")
                .bind(pagination.page_id.as_str())
                .bind(pagination.page_size)
                .fetch_all(&state.db_pool)
                .await,
        )
    })
    .await
}

async fn insert_note(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(note): Json<Note>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        sqlx::query("INSERT INTO Note (id, title, content, last_modified) VALUES (?, ?, ?, ?);")
            .bind(note.id)
            .bind(note.title)
            .bind(note.content)
            .bind(note.last_modified)
            .execute(&state.db_pool)
            .await
            .to_json_result("Inserted the note successfully!".to_string())
    })
    .await
}

async fn delete_all_notes(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        transaction_execute(
            &state.db_pool,
            sqlx::query("DELETE FROM Note;"),
            "Couldn't delete all notes!",
            "Couldn't delete all notes!",
            "Deleted all the notes successfully",
        )
        .await
    })
    .await
}

async fn update_a_note(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(note): Json<Note>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        sqlx::query("UPDATE Note SET title = ?, content = ?, last_modified = ? WHERE id = ?;")
            .bind(note.title)
            .bind(note.content)
            .bind(note.last_modified)
            .bind(note.id)
            .execute(&state.db_pool)
            .await
            .to_json_result("Updated the note successfully!".to_string())
    })
    .await
}

async fn delete_a_note(
    State(state): State<LimaServerState>,
    AuthBearer(bearer_token): AuthBearer,
    Json(id_dto): Json<IdDTO>,
) -> Result<Json<Value>, StatusCode> {
    run_if_valid_bearer(&bearer_token, || async move {
        sqlx::query("DELETE FROM Note WHERE id = ?;")
            .bind(id_dto.id)
            .execute(&state.db_pool)
            .await
            .to_json_result("Deleted the note successfully!".to_string())
    })
    .await
}
