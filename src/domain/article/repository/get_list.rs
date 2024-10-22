use std::sync::Arc;

use sqlx::PgPool;

use crate::{domain::article::entity::Article, global::errors::CustomError};

pub async fn get_list(pool: &PgPool) -> Result<Vec<Article>, Arc<CustomError>> {
    let rows = sqlx::query_as::<_, Article>("SELECT * FROM tb_article")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            let err_msg = format!("Error(GetArticles): {:?}", &e);
            tracing::error!("{}", err_msg);

            let err = match e {
                sqlx::Error::Database(_) => CustomError::DatabaseError(e),
                _ => CustomError::Unexpected(e.into()),
            };

            Arc::new(err)
        })?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use sqlx::postgres::PgPoolOptions;

    use crate::{config::database::create_connection_pool, domain::article::entity::Article};

    use super::get_list;

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
    async fn check_get_articles_success() {
        // Arrange: 테스트 사항 준비
        let pool = create_connection_pool().await;

        // Act: 함수 실행
        let result = get_list(&pool).await;
        let result = result.as_ref().map_err(|e| println!("{:?}", e)).unwrap();

        // Assert: 검증
        let rows = sqlx::query_as::<_, Article>("SELECT * FROM tb_article")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows.len(), result.len())
    }
}
