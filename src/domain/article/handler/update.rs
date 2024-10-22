use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::{
    domain::article::{dto::request::edit::EditArticleRequest, usecase::ArticleUsecase},
    global::errors::CustomError,
};

pub async fn update_article(
    Extension(usecase): Extension<Arc<dyn ArticleUsecase>>,
    Extension(user_id): Extension<i64>,
    Path(article_id): Path<i64>,
    Json(edit_req): Json<EditArticleRequest>,
) -> impl IntoResponse {
    if !edit_req.check_fields() {
        return CustomError::Invalidate("Article".to_string()).into_response();
    }

    match usecase.update_article(user_id, article_id, edit_req).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "성공"}))).into_response(),
        Err(err) => err.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{extract::Request, routing::put, Extension, Router};
    use http_body_util::BodyExt;
    use mockall::predicate;
    use serde_json::{to_string, Value};
    use tower::ServiceExt;

    use crate::{
        domain::article::{dto::request::edit::EditArticleRequest, usecase::ArticleUsecase},
        global::errors::CustomError,
        tests::mocks::tests::MockArticleUsecaseImpl,
    };

    use super::update_article;

    #[tokio::test]
    async fn check_update_article_success() {
        let user_id: i64 = 1;
        let article_id: i64 = 1;

        let edit_req =
            EditArticleRequest::new(Some("수정 제목".to_string()), Some("수정 내용".to_string()));
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_update_article()
            .with(
                predicate::eq(user_id),
                predicate::eq(article_id),
                predicate::eq(edit_req.clone()),
            )
            .returning(|_, _, _| Ok(()));

        let app = Router::new()
            .route("/api/v1/article/:article_id", put(update_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/api/v1/article/{}", article_id))
            .header("content-type", "application/json")
            .body(to_string(&edit_req).unwrap())
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

        assert_eq!(body_json.get("message").unwrap(), "성공")
    }

    #[tokio::test]
    async fn check_not_found() {
        let user_id: i64 = 1;
        let article_id: i64 = -32;

        let edit_req =
            EditArticleRequest::new(Some("수정 제목".to_string()), Some("수정 내용".to_string()));
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_update_article()
            .with(
                predicate::eq(user_id),
                predicate::eq(article_id),
                predicate::eq(edit_req.clone()),
            )
            .returning(|_, _, _| Err(Arc::new(CustomError::NotFound("Article".to_string()))));

        let app = Router::new()
            .route("/api/v1/article/:article_id", put(update_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/api/v1/article/{}", article_id))
            .header("content-type", "application/json")
            .body(to_string(&edit_req).unwrap())
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
        let user_id: i64 = 2;
        let article_id: i64 = 33;

        let edit_req =
            EditArticleRequest::new(Some("수정 제목".to_string()), Some("수정 내용".to_string()));
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_update_article()
            .with(
                predicate::eq(user_id),
                predicate::eq(article_id),
                predicate::eq(edit_req.clone()),
            )
            .returning(|_, _, _| Err(Arc::new(CustomError::Unauthorized("Article".to_string()))));

        let app = Router::new()
            .route("/api/v1/article/:article_id", put(update_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/api/v1/article/{}", article_id))
            .header("content-type", "application/json")
            .body(to_string(&edit_req).unwrap())
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

    #[tokio::test]
    async fn check_no_field_to_update() {
        let user_id: i64 = 1;
        let article_id: i64 = 1;

        let edit_req = EditArticleRequest::new(None, Some("".to_string()));
        let mock_usecase = MockArticleUsecaseImpl::new();

        let app = Router::new()
            .route("/api/v1/article/:article_id", put(update_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/api/v1/article/{}", article_id))
            .header("content-type", "application/json")
            .body(to_string(&edit_req).unwrap())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 400);

        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        println!("{}", &body_str);

        assert_eq!(body_str, "Validation failed")
    }
}
