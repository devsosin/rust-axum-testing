#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use axum::async_trait;
    use mockall::mock;

    use crate::{
        domain::article::{
            dto::{request::create::ArticleCreateRequest, response::create::ArticleCreateResponse},
            entity::Article,
            repository::ArticleRepository,
            usecase::ArticleUsecase,
        },
        global::errors::CustomError,
    };

    mock! {
        pub ArticleRepositoryImpl {}

        #[async_trait]
        impl ArticleRepository for ArticleRepositoryImpl {
            async fn save_article(&self, article: Article) -> Result<i64, Arc<CustomError>>;
        }
    }

    mock! {
        pub ArticleUsecaseImpl {}

        #[async_trait]
        impl ArticleUsecase for ArticleUsecaseImpl {
            async fn create_article(
                &self,
                user_id: i64,
                create_req: ArticleCreateRequest,
            ) -> Result<ArticleCreateResponse, Arc<CustomError>>;
        }

    }
}
