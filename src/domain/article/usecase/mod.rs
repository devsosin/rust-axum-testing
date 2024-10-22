use std::sync::Arc;

use axum::async_trait;
use create::create_article;

use crate::global::errors::CustomError;

use super::{
    dto::{request::create::ArticleCreateRequest, response::create::ArticleCreateResponse},
    repository::ArticleRepository,
};

mod create;
mod delete;
mod read_article;
mod read_articles;
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
pub trait ArticleUsecase: Send + Sync {
    async fn create_article(
        &self,
        user_id: i64,
        create_req: ArticleCreateRequest,
    ) -> Result<ArticleCreateResponse, Arc<CustomError>>;
}

#[async_trait]
impl ArticleUsecase for ArticleUsecaseImpl {
    async fn create_article(
        &self,
        user_id: i64,
        create_req: ArticleCreateRequest,
    ) -> Result<ArticleCreateResponse, Arc<CustomError>> {
        create_article(self.repository.clone(), user_id, create_req).await
    }
}
