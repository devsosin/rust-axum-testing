use std::sync::Arc;

use axum::async_trait;

use super::repository::ArticleRepository;

mod create;
mod delete;
mod read_article;
mod update;

// 구조체
pub struct ArticleUsecaseImpl {
    repository: Arc<dyn ArticleRepository>,
}

impl ArticleUsecaseImpl {
    pub fn new(repository: Arc<dyn ArticleRepository>) -> Self {
        Self { repository }
    }
}

// 트레잇
#[async_trait]
pub trait ArticleUsecase: Send + Sync {}

#[async_trait]
impl ArticleUsecase for ArticleUsecaseImpl {}
