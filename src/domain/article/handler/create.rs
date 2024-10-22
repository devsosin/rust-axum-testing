use axum::response::IntoResponse;

pub async fn create_article() -> impl IntoResponse {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{extract::Request, routing::post, Extension, Router};
    use http_body_util::BodyExt;
    use mockall::predicate;
    use serde_json::{to_string, Value};
    use tower::ServiceExt;

    use crate::{
        domain::article::{
            dto::{request::create::ArticleCreateRequest, response::create::ArticleCreateResponse},
            usecase::ArticleUsecase,
        },
        global::errors::CustomError,
        tests::mocks::tests::MockArticleUsecaseImpl,
    };

    use super::create_article;

    #[tokio::test]
    async fn check_create_article_success() {
        // Arrange: 생성 데이터 준비
        let title = "테스트용 제목 1";
        let content = "테스트용 내용 1";
        let create_req = ArticleCreateRequest::new(title.to_string(), content.to_string());
        let user_id: i64 = 1;
        let inserted_id = 1;

        // Mocking
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_create_article()
            .with(predicate::eq(user_id), predicate::eq(create_req.clone()))
            .returning(move |_, _| Ok(ArticleCreateResponse::new(inserted_id)));

        // 요청을 받을 임시 app 준비
        let app = Router::new()
            .route("/api/v1/article", post(create_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));

        // 요청 생성
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/article")
            .header("content-type", "application/json")
            .body(to_string(&create_req).unwrap())
            .unwrap();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert: status_code 검증
        assert_eq!(response.status(), 201);

        // body
        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");

        // 디버깅 시 사용
        println!("{:?}", &body_str);

        let body_json: Value =
            serde_json::from_str(&body_str).expect("failed to parse String to json");

        let response_id = body_json.get("data").unwrap().get("id").unwrap().as_i64();

        // Assert: body 검증
        assert_eq!(response_id.unwrap(), inserted_id)
    }

    #[tokio::test]
    async fn check_title_is_empty() {
        // Arrange: 생성 데이터 준비
        let title = "";
        let content = "테스트용 내용 1";
        let create_req = ArticleCreateRequest::new(title.to_string(), content.to_string());
        let user_id: i64 = 1;

        let mock_usecase = MockArticleUsecaseImpl::new();

        // 요청을 받을 임시 app 준비
        let app = Router::new()
            .route("/api/v1/article", post(create_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(user_id));

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/article")
            .header("content-type", "application/json")
            .body(to_string(&create_req).unwrap())
            .unwrap();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // status_code
        assert_eq!(response.status(), 400);

        // body
        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        // 디버깅 시 사용
        println!("{:?}", &body_str);

        assert_eq!(&body_str, "Validation failed")
    }

    #[tokio::test]
    async fn check_database_error() {
        // Arrange: 생성 데이터 준비
        let title = "DB 에러용 제목 1";
        let content = "테스트용 내용 1";
        let create_req = ArticleCreateRequest::new(title.to_string(), content.to_string());
        let no_user_id: i64 = -32;

        // Mocking
        let mut mock_usecase = MockArticleUsecaseImpl::new();
        mock_usecase
            .expect_create_article()
            .with(predicate::eq(no_user_id), predicate::eq(create_req.clone()))
            .returning(|_, _| {
                Err(Arc::new(CustomError::DatabaseError(
                    sqlx::Error::RowNotFound, // 실제 에러는 아니지만 대체할 임시 에러 생성
                )))
            });

        // 요청을 받을 임시 app 준비
        let app = Router::new()
            .route("/api/v1/article", post(create_article))
            .layer(Extension(Arc::new(mock_usecase) as Arc<dyn ArticleUsecase>))
            .layer(Extension(no_user_id));

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/article")
            .header("content-type", "application/json")
            .body(to_string(&create_req).unwrap())
            .unwrap();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // status_code
        assert_eq!(response.status(), 400);

        // body
        let body = response.into_body();
        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");
        // 디버깅 시 사용
        println!("{:?}", &body_str);

        assert_eq!(&body_str, "Database error")
    }
}
