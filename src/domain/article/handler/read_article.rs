use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::domain::article::usecase::ArticleUsecase;

pub async fn read_article(
    Extension(usecase): Extension<Arc<dyn ArticleUsecase>>,
    Extension(user_id): Extension<i64>,
    Path(article_id): Path<i64>,
) -> impl IntoResponse {
    match usecase.read_article(article_id).await {
        Ok(res) => (
            StatusCode::OK,
            Json(json!({"message": "성공", "data": res})),
        )
            .into_response(),
        Err(err) => err.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, extract::Request, routing::get, Extension, Router};
    use http_body_util::BodyExt;
    use mockall::predicate;
    use serde_json::Value;
    use tower::ServiceExt;

    use crate::{
        domain::article::{
            dto::response::read_article::ReadArticleResponse, usecase::ArticleUsecase,
        },
        global::errors::CustomError,
        tests::mocks::tests::MockArticleUsecaseImpl,
    };

    use super::read_article;

    #[tokio::test]
    async fn check_read_article_success() {
        // Arrange
        let article_id: i64 = 1;

        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_read_article()
            .with(predicate::eq(article_id))
            .returning(|id| {
                Ok(ReadArticleResponse::new(
                    id,
                    "임시 제목".to_string(),
                    "임시 내용".to_string(),
                    1,
                ))
            });

        let app = Router::new()
            .route("/api/v1/article/:article_id", get(read_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(1 as i64));

        let req = Request::builder()
            .method("GET")
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
            serde_json::from_str(&body_str).expect("failed to parse body to JSON");

        assert_eq!(body_json.get("message").unwrap(), "성공");
        assert_eq!(
            body_json
                .get("data")
                .unwrap()
                .get("id")
                .unwrap()
                .as_i64()
                .unwrap(),
            article_id
        );
    }

    #[tokio::test]
    async fn check_not_found() {
        // Arrange
        let article_id: i64 = -32;

        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_read_article()
            .with(predicate::eq(article_id))
            .returning(|_| Err(Arc::new(CustomError::NotFound("Article".to_string()))));

        let app = Router::new()
            .route("/api/v1/article/:article_id", get(read_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(1 as i64));

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/article/{}", article_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 404);
    }
}
