use std::sync::Arc;

use axum::async_trait;
use sqlx::PgPool;

mod delete;
mod get_detail;
mod save;
mod update;

pub struct ArticleRepositoryImpl {
    pool: Arc<PgPool>,
}

impl ArticleRepositoryImpl {
    pub fn new(pool: &Arc<PgPool>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
pub trait ArticleRepository: Send + Sync {}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {}
