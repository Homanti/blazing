use std::env;
use std::error::Error;
use std::sync::Arc;
use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use blazing_auth::{create_auth_routes, AuthService};
use blazing_chat::{create_message_routes, MessageService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .init();

    let database_url = env::var("DATABASE_URL")?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connection established");

    let jwt_secret = env::var("JWT_SECRET")?;

    let auth_service = Arc::new(AuthService::new(db_pool.clone(), jwt_secret));

    let message_service = Arc::new(MessageService::new(db_pool.clone()));

    let api_router = Router::new()
        .nest("/auth", create_auth_routes(auth_service.clone()))
        .nest("/message", create_message_routes(message_service, auth_service.clone()));

    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1", api_router)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

    tracing::info!("Server shutting down...");
    tracing::debug!("Active database connections: {}", db_pool.size());
    db_pool.close().await;
    tracing::info!("Database connection pool closed");

    Ok(())
}

async fn shutdown_signal() {
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

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Received shutdown signal");

}

async fn root() -> &'static str {
    "API online"
}