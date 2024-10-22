// Rust Base Library
use std::sync::Arc;

// Axum
use axum::{http::StatusCode, Router};

// Env
use dotenv::dotenv;

// Logging
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Domain Routers
use domain::article::route::get_router as article_router;

// User Defined Modules
pub mod config {
    pub mod database;
}

pub mod middleware {}

pub mod domain {
    pub mod article;
}

pub mod global {
    pub mod errors;
}

#[cfg(test)]
pub mod tests {
    pub mod mocks;
}

#[tokio::main]
async fn main() {
    // 환경변수 로드
    dotenv().ok();

    // 로깅 설정
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = config::database::create_connection_pool().await;
    let pool = Arc::new(pool);

    let article_router = article_router(&pool);

    let app = Router::new()
        .route("/", axum::routing::get(|| async { "{\"status\": \"OK\"}" }))
        .nest("/api/v1/article", article_router);

    let app = app.fallback(|| async { (StatusCode::NOT_FOUND, "API NOT FOUND") });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
