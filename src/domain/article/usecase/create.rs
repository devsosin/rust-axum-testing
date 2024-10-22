use std::sync::Arc;

use crate::{
    domain::article::{
        dto::{request::create::ArticleCreateRequest, response::create::ArticleCreateResponse},
        repository::ArticleRepository,
    },
    global::errors::CustomError,
};

pub async fn create_article(
    repository: Arc<dyn ArticleRepository>,
    user_id: i64,
    create_req: ArticleCreateRequest,
) -> Result<ArticleCreateResponse, Arc<CustomError>> {
    let new_article = create_req.to_entity(user_id);
    let id = repository.save_article(new_article).await?;

    Ok(ArticleCreateResponse::new(id))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;

    use crate::domain::article::dto::request::create::ArticleCreateRequest;

    use crate::tests::mocks::tests::MockArticleRepositoryImpl;

    use super::create_article;

    #[tokio::test]
    async fn check_create_article_success() {
        // Arrange
        let create_req =
            ArticleCreateRequest::new("테스트 제목 1".to_string(), "테스트 내용 1".to_string());

        let user_id = 1;

        // mock 동작 설정
        let mut mock_repo = MockArticleRepositoryImpl::new();
        let inserted_id = 1;
        mock_repo
            .expect_save_article()
            .with(predicate::eq(create_req.to_entity(user_id)))
            .returning(move |_| Ok(inserted_id));

        // Act
        let result = create_article(Arc::new(mock_repo), user_id, create_req).await;
        let create_res = result.map_err(|e| format!("{:?}", e)).unwrap();

        // Assert
        assert_eq!(create_res.get_id(), inserted_id)
    }
}
