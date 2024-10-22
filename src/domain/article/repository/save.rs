use std::sync::Arc;

use sqlx::PgPool;

use crate::global::errors::CustomError;

pub async fn save_article(pool: &PgPool) -> Result<i64, Arc<CustomError>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use sqlx::postgres::PgPoolOptions;

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
}
