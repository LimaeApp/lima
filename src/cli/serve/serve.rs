use crate::LIMAE_DIR_PATH;
use crate::route::{apps_router, dictionary_router, notes_router};
use axum::Router;
use local_ip_address::local_ip;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Clone)]
pub struct LimaServerState {
    pub db_pool: SqlitePool,
}

pub async fn serve() {
    if !tokio::fs::try_exists(&*LIMAE_DIR_PATH)
        .await
        .expect("Issue while locating limae specific directory")
    {
        tokio::fs::create_dir(&*LIMAE_DIR_PATH)
            .await
            .expect("Couldn't create limae specific directory");
    }

    let db_pool = setup_and_get_db(format!("{}/cli.db", &*LIMAE_DIR_PATH)).await;

    let lima_server_state: LimaServerState = LimaServerState { db_pool };

    let lima_router = Router::new()
        .nest("/notes", notes_router::create())
        .nest("/dictionary", dictionary_router::create())
        .nest("/apps", apps_router::create())
        .with_state(lima_server_state.clone());

    let bind_address = format!("{}:3000", local_ip().unwrap().to_string());
    let tcp_listener = TcpListener::bind(&bind_address).await.unwrap();

    println!("Listening at {bind_address}");

    axum::serve(tcp_listener, lima_router)
        .with_graceful_shutdown(on_shutdown(|| async move {
            lima_server_state.db_pool.close().await
        }))
        .await
        .unwrap();
}

pub async fn setup_and_get_db(path: String) -> SqlitePool {
    let connection_string = format!("sqlite://{}", path);
    let db_options = SqliteConnectOptions::from_str(&connection_string)
        .unwrap()
        .create_if_missing(true);

    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_options)
        .await
        .expect("Couldn't create SQLite connection pool");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS AppBlocklist (
            id TEXT PRIMARY KEY,
            package_name TEXT NOT NULL UNIQUE,
            CHECK(trim(package_name) <> '' AND trim(id) <> '')
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Couldn't create the AppBlocklist table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Dictionary (
            string TEXT NOT NULL UNIQUE,
            id TEXT PRIMARY KEY,
            CHECK(trim(string) <> '' AND trim(id) <> '')
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Couldn't create the Dictionary table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Note (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            last_modified INTEGER NOT NULL,
            CHECK((trim(title) <> '' OR trim(content) <> '') = 1 AND trim(id) <> '')
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Couldn't create the Note table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Tombstone(
           id TEXT PRIMARY KEY,
           originTable TEXT NOT NULL,
           originId TEXT NOT NULL,
           deletedAt INTEGER NOT NULL,
           CHECK(trim(id) <> '' AND trim(originTable) <> '' AND trim(originId) <> '')
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Couldn't create Tombstone Table");

    db_pool
}

async fn on_shutdown<Callback, Deferred>(cleanup: Callback)
where
    Callback: FnOnce() -> Deferred,
    Deferred: Future,
{
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let cleanup = async || {
        println!("Cleaning up resources...");
        cleanup().await;
        println!("Shutting down...");
    };

    tokio::select! {
        _ = ctrl_c => cleanup().await,
        _ = terminate => cleanup().await,
    }
}
