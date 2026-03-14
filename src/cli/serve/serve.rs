use crate::LIMAE_DIR_PATH;
use crate::route::{apps_router, dictionary_router, notes_router};
use axum::Router;
use local_ip_address::local_ip;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::fs;
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Clone)]
pub struct LimaServerState {
    pub db_pool: SqlitePool,
}

#[tokio::main]
pub async fn serve() {
    if !fs::exists(&*LIMAE_DIR_PATH).expect("Issue while locating limae specific directory") {
        fs::create_dir(&*LIMAE_DIR_PATH).expect("Couldn't create limae specific directory");
    }

    let db_pool = setup_and_get_db(format!("{}/cli.db", &*LIMAE_DIR_PATH)).await;

    let lima_server_state: LimaServerState = LimaServerState { db_pool };

    let lima_router = Router::new()
        .nest("/notes", notes_router::create())
        .nest("/dictionary", dictionary_router::create())
        .nest("/apps", apps_router::create())
        .with_state(lima_server_state.clone());

    let mut ipv4addr: Option<String> = Some("0.0.0.0:3000".parse().unwrap());
    let local_ips = local_ip();

    for ip in local_ips.iter() {
        if ip.is_ipv4() {
            ipv4addr = Some(format!("{ip}:3000"));
            break;
        }
    }

    let ipv4_as_string = &ipv4addr.unwrap();

    let tcp_listener = TcpListener::bind(ipv4_as_string).await.unwrap();

    println!("Listening at {ipv4_as_string}");

    axum::serve(tcp_listener, lima_router)
        .with_graceful_shutdown(on_shutdown(|| async move {
            lima_server_state.db_pool.close().await;
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
            packageName TEXT NOT NULL UNIQUE,
            CHECK(trim(packageName) <> '' AND trim(id) <> '')
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
            lastModified INTEGER NOT NULL,
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
