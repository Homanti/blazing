use std::env;
use std::error::Error;
use std::sync::Arc;
use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use blazing_auth::{create_auth_routes, AuthService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Database connection established");

    let jwt_secret = env::var("JWT_SECRET")?;

    let auth_service = Arc::new(AuthService::new(db_pool.clone(), jwt_secret));

    let app = Router::new()
        .route("/", get(root))
        .nest("/auth", create_auth_routes(auth_service));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    db_pool.close().await;

    Ok(())
}

async fn root() -> &'static str {
    "API online"
}