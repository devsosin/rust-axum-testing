use std::sync::Arc;

use sqlx::PgPool;

use crate::{domain::article::entity::Article, global::errors::CustomError};

#[derive(Debug, sqlx::FromRow)]
struct InsertResult {
    id: i64,
}

pub async fn save_article(pool: &PgPool, article: Article) -> Result<i64, Arc<CustomError>> {
    let row = sqlx::query_as::<_, InsertResult>(
        r#"INSERT INTO tb_article (title, content, writer_id)
        VALUES ($1, $2, $3)
        RETURNING id;
        "#,
    )
    .bind(article.get_title())
    .bind(article.get_content())
    .bind(article.get_writer())
    .fetch_one(pool)
    .await
    .map_err(|e| {
        let err_msg = format!("Error(SaveArticle): {:?}", &e);
        tracing::error!("{}", err_msg);

        let err = match e {
            sqlx::Error::Database(_) => CustomError::DatabaseError(e),
            _ => CustomError::Unexpected(e.into()),
        };

        Arc::new(err)
    })?;

    Ok(row.id)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use sqlx::postgres::PgPoolOptions;

    use crate::{
        config::database::create_connection_pool, domain::article::entity::Article,
        global::errors::CustomError,
    };

    use super::save_article;

    #[tokio::test]
    async fn check_database_connectivity() {
        // Arrange: 테스트 사항 준비
        let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

        // Act: 테스트 진행
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("Unable to connect to database");

        // Assert: 결과 검증
        assert_eq!(pool.is_closed(), false)
    }

    #[tokio::test]
    async fn check_save_article_success() {
        // Arrange: 데이터베이스 pool 생성, 저장할 게시글 하나 준비
        let pool = create_connection_pool().await;
        let user_id = 1;
        let article = Article::new(
            "테스트 제목 1".to_string(),
            "테스트 내용 1".to_string(),
            user_id,
        );

        // Act: 테스트 실행
        let result = save_article(&pool, article.clone()).await;

        let inserted_id = result.as_ref().map_err(|e| format!("{:?}", e)).unwrap();

        // Assert: 데이터베이스에 삽입되었는지 확인
        let row = sqlx::query_as::<_, Article>("SELECT * FROM tb_article WHERE id = $1")
            .bind(inserted_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(article.get_title(), row.get_title());
        assert_eq!(article.get_content(), row.get_content());
    }

    #[tokio::test]
    async fn check_writer_not_found() {
        // Arrange: 데이터베이스 pool 생성, 저장할 게시글 하나 준비
        let pool = create_connection_pool().await;
        let no_user_id = -32;
        let article = Article::new(
            "테스트 제목 1".to_string(),
            "테스트 내용 1".to_string(),
            no_user_id,
        );

        // Act: 테스트 실행
        let result = save_article(&pool, article.clone()).await;

        // Assert: 에러 발생해야함
        assert!(result.is_err());

        // foreign_key의 무결성 제약 위반과 같은 에러는 데이터베이스 에러로 빠짐
        let err_type = match *result.err().unwrap() {
            CustomError::DatabaseError(_) => true,
            _ => false,
        };
        assert!(err_type)
    }
}
