use crate::route::{apps_router, dictionary_router, notes_router};
use axum::Router;
use local_ip_address::local_ip;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Clone)]
pub struct LimaServerState;

#[tokio::main]
pub async fn serve() {
    let lima_router = Router::new()
        .nest("/notes", notes_router::create())
        .nest("/dictionary", dictionary_router::create())
        .nest("/apps", apps_router::create())
        .with_state(Arc::new(LimaServerState));

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
        .with_graceful_shutdown(on_shutdown())
        .await
        .unwrap();
}

async fn on_shutdown() {
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

    fn cleanup() {
        println!("Shutting down...")
    }

    tokio::select! {
        _ = ctrl_c => cleanup(),
        _ = terminate => cleanup(),
    }
}
