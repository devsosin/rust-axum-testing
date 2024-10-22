use std::sync::Arc;

use crate::{
    domain::article::{
        dto::response::read_article::ReadArticleResponse, repository::ArticleRepository,
    },
    global::errors::CustomError,
};

pub async fn read_article(
    repository: Arc<dyn ArticleRepository>,
    article_id: i64,
) -> Result<ReadArticleResponse, Arc<CustomError>> {
    let article = repository.get_detail(article_id).await?;

    Ok(article.to_response())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;

    use crate::{
        domain::article::entity::Article, global::errors::CustomError,
        tests::mocks::tests::MockArticleRepositoryImpl,
    };

    use super::read_article;

    #[tokio::test]
    async fn check_read_article_success() {
        // Arrange
        let article_id = 1;
        let mut mock_repo = MockArticleRepositoryImpl::new();
        mock_repo
            .expect_get_detail()
            .with(predicate::eq(article_id))
            .returning(|id| Ok(Article::new("제목".to_string(), "내용".to_string(), 1).id(id)));

        // Act
        let result = read_article(Arc::new(mock_repo), article_id).await;
        let result = result.map_err(|e| println!("{:?}", e)).unwrap();

        assert_eq!(result.get_id(), article_id)
    }

    #[tokio::test]
    async fn check_not_found() {
        // Arrange
        let article_id = 1;
        let mut mock_repo = MockArticleRepositoryImpl::new();
        mock_repo
            .expect_get_detail()
            .with(predicate::eq(article_id))
            .returning(|_| Err(Arc::new(CustomError::NotFound("Article".to_string()))));

        // Act
        let result = read_article(Arc::new(mock_repo), article_id).await;

        assert!(result.is_err());

        let err_type = match *result.err().unwrap() {
            CustomError::NotFound(_) => true,
            _ => false,
        };
        assert!(err_type)
    }
}
