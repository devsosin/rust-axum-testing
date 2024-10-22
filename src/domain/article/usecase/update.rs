use std::sync::Arc;

use crate::{
    domain::article::{dto::request::edit::EditArticleRequest, repository::ArticleRepository},
    global::errors::CustomError,
};

pub async fn update_article(
    repository: Arc<dyn ArticleRepository>,
    user_id: i64,
    article_id: i64,
    edit_req: EditArticleRequest,
) -> Result<(), Arc<CustomError>> {
    let (title, content) = edit_req.to_fields();

    repository
        .update_article(user_id, article_id, title, content)
        .await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;

    use crate::{
        domain::article::dto::request::edit::EditArticleRequest,
        tests::mocks::tests::MockArticleRepositoryImpl,
    };

    use super::update_article;

    #[tokio::test]
    async fn check_update_article_success() {
        let user_id: i64 = 1;
        let article_id: i64 = 1;

        let mut mock_repo = MockArticleRepositoryImpl::new();

        let title_update = Some("수정 제목".to_string());
        let content_update = Some("수정 내용".to_string());

        let edit_req = EditArticleRequest::new(title_update.clone(), content_update.clone());

        mock_repo
            .expect_update_article()
            .with(
                predicate::eq(user_id),
                predicate::eq(article_id),
                predicate::eq(title_update),
                predicate::eq(content_update),
            )
            .returning(|_, _, _, _| Ok(()));

        let result = update_article(Arc::new(mock_repo), user_id, article_id, edit_req).await;
        result.map_err(|e| println!("{:?}", e)).unwrap();
    }
}
