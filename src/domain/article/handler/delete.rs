use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::domain::article::usecase::ArticleUsecase;

pub async fn delete_article(
    Extension(usecase): Extension<Arc<dyn ArticleUsecase>>,
    Extension(user_id): Extension<i64>,
    Path(article_id): Path<i64>,
) -> impl IntoResponse {
    match usecase.delete_article(user_id, article_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "성공"}))).into_response(),
        Err(err) => err.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, extract::Request, routing::delete, Extension, Router};
    use http_body_util::BodyExt;
    use mockall::predicate;
    use serde_json::Value;
    use tower::ServiceExt;

    use crate::{
        domain::article::usecase::ArticleUsecase, global::errors::CustomError,
        tests::mocks::tests::MockArticleUsecaseImpl,
    };

    use super::delete_article;

    #[tokio::test]
    async fn check_delete_article_success() {
        let user_id = 1;
        let article_id = 1;
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_delete_article()
            .with(predicate::eq(user_id), predicate::eq(article_id))
            .returning(|_, _| Ok(()));

        let app = Router::new()
            .route("/api/v1/article/:article_id", delete(delete_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));

        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/article/{}", article_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 200);

        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        println!("{}", &body_str);

        let body_json: Value =
            serde_json::from_str(&body_str).expect("failed to parse body to json");

        assert_eq!(body_json.get("message").unwrap(), "성공")
    }

    #[tokio::test]
    async fn check_not_found() {
        let user_id = 1;
        let article_id = -32;
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_delete_article()
            .with(predicate::eq(user_id), predicate::eq(article_id))
            .returning(|_, _| Err(Arc::new(CustomError::NotFound("Article".to_string()))));

        let app = Router::new()
            .route("/api/v1/article/:article_id", delete(delete_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));

        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/article/{}", article_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 404);

        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        println!("{}", &body_str);

        assert_eq!(body_str, "Article not found")
    }

    #[tokio::test]
    async fn check_no_authority() {
        let user_id = 3;
        let article_id = 1;
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_delete_article()
            .with(predicate::eq(user_id), predicate::eq(article_id))
            .returning(|_, _| Err(Arc::new(CustomError::Unauthorized("Article".to_string()))));

        let app = Router::new()
            .route("/api/v1/article/:article_id", delete(delete_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));

        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/article/{}", article_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 401);

        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        println!("{}", &body_str);

        assert_eq!(body_str, "Authorization failed")
    }
}
