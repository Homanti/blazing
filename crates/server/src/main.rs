use std::env;
use std::error::Error;
use std::sync::Arc;
use axum::{Router, routing::get};
use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use tokio::net::TcpListener;
use tokio::runtime::Handle;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use blazing_auth::{create_auth_routes, AuthService};
use blazing_chat::{MessagesService, create_chat_routes, WsMessage};
use blazing_ws::Broadcaster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://192.168.86.246:5173".parse().unwrap(),
            "https://blazing.up.railway.app".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(true);

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL")?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connection established");

    let jwt_secret = env::var("JWT_SECRET")?;

    let auth_service = Arc::new(AuthService::new(db_pool.clone(), jwt_secret.clone()));
    let broadcaster = Arc::new(Broadcaster::<Uuid, WsMessage>::new());
    let messages_service = Arc::new(MessagesService::new(
        db_pool.clone(),
        broadcaster.clone()
    ));
        
    let api_routes = Router::new()
        .nest("/auth", create_auth_routes(auth_service.clone()))
        .nest("/chat", create_chat_routes(
            messages_service,
            auth_service.clone(),
            broadcaster,
        ));

    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1", api_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)))
        .layer(cors);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("Server running on http://0.0.0.0:{port}");

    let metrics = Handle::current().metrics();
    let workers = metrics.num_workers();
    tracing::info!("Tokio runtime using {} worker threads", workers);

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