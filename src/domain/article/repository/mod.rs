use std::sync::Arc;

use axum::async_trait;
use delete::delete_article;
use get_detail::get_detail;
use get_list::get_list;
use save::save_article;
use sqlx::PgPool;
use update::update_article;

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
    async fn get_list(&self) -> Result<Vec<Article>, Arc<CustomError>>;
    async fn get_detail(&self, article_id: i64) -> Result<Article, Arc<CustomError>>;
    async fn delete_article(&self, user_id: i64, article_id: i64) -> Result<(), Arc<CustomError>>;
    async fn update_article(
        &self,
        user_id: i64,
        article_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<(), Arc<CustomError>>;
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {
    async fn save_article(&self, article: Article) -> Result<i64, Arc<CustomError>> {
        save_article(&self.pool, article).await
    }
    async fn get_list(&self) -> Result<Vec<Article>, Arc<CustomError>> {
        get_list(&self.pool).await
    }

    async fn get_detail(&self, article_id: i64) -> Result<Article, Arc<CustomError>> {
        get_detail(&self.pool, article_id).await
    }

    async fn delete_article(&self, user_id: i64, article_id: i64) -> Result<(), Arc<CustomError>> {
        delete_article(&self.pool, user_id, article_id).await
    }
    async fn update_article(
        &self,
        user_id: i64,
        article_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<(), Arc<CustomError>> {
        update_article(&self.pool, user_id, article_id, title, content).await
    }
}
