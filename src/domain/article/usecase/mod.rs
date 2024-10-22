use std::sync::Arc;

use axum::async_trait;
use create::create_article;
use delete::delete_article;
use read_article::read_article;
use update::update_article;

use crate::global::errors::CustomError;

use super::{
    dto::{
        request::{create::ArticleCreateRequest, edit::EditArticleRequest},
        response::{create::ArticleCreateResponse, read_article::ReadArticleResponse},
    },
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
    async fn read_article(&self, article_id: i64) -> Result<ReadArticleResponse, Arc<CustomError>>;
    async fn update_article(
        &self,
        user_id: i64,
        article_id: i64,
        edit_req: EditArticleRequest,
    ) -> Result<(), Arc<CustomError>>;
    async fn delete_article(&self, user_id: i64, article_id: i64) -> Result<(), Arc<CustomError>>;
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
    async fn read_article(&self, article_id: i64) -> Result<ReadArticleResponse, Arc<CustomError>> {
        read_article(self.repository.clone(), article_id).await
    }
    async fn update_article(
        &self,
        user_id: i64,
        article_id: i64,
        edit_req: EditArticleRequest,
    ) -> Result<(), Arc<CustomError>> {
        update_article(self.repository.clone(), user_id, article_id, edit_req).await
    }
    async fn delete_article(&self, user_id: i64, article_id: i64) -> Result<(), Arc<CustomError>> {
        delete_article(self.repository.clone(), user_id, article_id).await
    }
}
