use std::sync::Arc;

use axum::async_trait;
use save::save_article;
use sqlx::PgPool;

use crate::global::errors::CustomError;

use super::entity::Article;

mod delete;
mod get_detail;
mod get_list;
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
pub trait ArticleRepository: Send + Sync {
    async fn save_article(&self, article: Article) -> Result<i64, Arc<CustomError>>;
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {
    async fn save_article(&self, article: Article) -> Result<i64, Arc<CustomError>> {
        save_article(&self.pool, article).await
    }
}
