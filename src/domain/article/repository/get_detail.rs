use std::sync::Arc;

use sqlx::PgPool;

use crate::{domain::article::entity::Article, global::errors::CustomError};

pub async fn get_detail(pool: &PgPool, article_id: i64) -> Result<Article, Arc<CustomError>> {
    let row = sqlx::query_as::<_, Article>("SELECT * FROM tb_article WHERE id = $1")
        .bind(article_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            let err_msg = format!("Error(GetArticle): {:?}", &e);
            tracing::error!("{}", err_msg);

            let err = match e {
                sqlx::Error::Database(_) => CustomError::DatabaseError(e),
                sqlx::Error::RowNotFound => CustomError::NotFound("Article".to_string()),
                _ => CustomError::Unexpected(e.into()),
            };

            Arc::new(err)
        })?;

    Ok(row)
}

#[cfg(test)]
mod tests {
    use crate::{
        config::database::create_connection_pool,
        domain::article::{entity::Article, repository::save::save_article},
        global::errors::CustomError,
    };

    use super::get_detail;

    #[tokio::test]
    async fn check_database_connectivity() {
        let pool = create_connection_pool().await;

        assert_eq!(pool.is_closed(), false)
    }

    #[tokio::test]
    async fn check_get_article_success() {
        let pool = create_connection_pool().await;

        let new_article = Article::new("테스트 제목1".to_string(), "테스트 내용1".to_string(), 1);
        let article_id = save_article(&pool, new_article.clone()).await.unwrap();

        let result = get_detail(&pool, article_id).await;
        let result = result.map_err(|e| println!("{:?}", e)).unwrap();

        assert_eq!(result.get_title(), new_article.get_title());
        assert_eq!(result.get_content(), new_article.get_content())
    }

    #[tokio::test]
    async fn check_article_not_found() {
        let pool = create_connection_pool().await;

        let no_id = -32;

        let result = get_detail(&pool, no_id).await;

        assert!(result.as_ref().is_err());
        let err_type = match *result.err().unwrap() {
            CustomError::NotFound(_) => true,
            _ => false,
        };
        assert!(err_type)
    }
}
