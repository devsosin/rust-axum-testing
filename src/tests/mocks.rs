#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use axum::async_trait;
    use mockall::mock;

    use crate::{
        domain::article::{entity::Article, repository::ArticleRepository},
        global::errors::CustomError,
    };

    mock! {
        pub ArticleRepositoryImpl {}

        #[async_trait]
        impl ArticleRepository for ArticleRepositoryImpl {
            async fn save_article(&self, article: Article) -> Result<i64, Arc<CustomError>>;
        }
    }
}
