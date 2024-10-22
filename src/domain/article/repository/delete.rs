use std::sync::Arc;

use sqlx::PgPool;

use crate::global::errors::CustomError;

#[derive(Debug, sqlx::FromRow)]
struct DeleteResult {
    is_exist: bool,
    is_authorized: bool,
    delete_count: i64,
}

pub async fn delete_article(
    pool: &PgPool,
    user_id: i64,
    article_id: i64,
) -> Result<(), Arc<CustomError>> {
    let result = sqlx::query_as::<_, DeleteResult>(
        r#"
        WITH ArticleExists AS (
            SELECT id, writer_id
            FROM tb_article
            WHERE id = $1
        ),
        AuthorityChecks AS (
            SELECT EXISTS (
                SELECT 1
                FROM ArticleExists
                WHERE writer_id = $2
            ) AS is_authorized
        ),
        DeleteArticle AS (
            DELETE FROM tb_article 
            WHERE id = $1 
                AND (SELECT is_authorized FROM AuthorityChecks) = true
            RETURNING id
        )
        SELECT
            EXISTS (SELECT 1 FROM ArticleExists) AS is_exist,
            (SELECT is_authorized FROM AuthorityChecks) AS is_authorized,
            (SELECT COUNT(*) FROM DeleteArticle) AS delete_count;
        "#,
    )
    .bind(article_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        let err_msg = format!("Error(DeleteArticle): {:?}", &e);
        tracing::error!("{}", err_msg);

        let err = match e {
            sqlx::Error::Database(_) => CustomError::DatabaseError(e),
            _ => CustomError::Unexpected(e.into()),
        };

        Arc::new(err)
    })?;

    if !result.is_exist {
        return Err(Arc::new(CustomError::NotFound("Article".to_string())));
    } else if !result.is_authorized {
        return Err(Arc::new(CustomError::Unauthorized("Article".to_string())));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        config::database::create_connection_pool,
        domain::article::{
            entity::Article,
            repository::{get_detail::get_detail, save::save_article},
        },
        global::errors::CustomError,
    };

    use super::delete_article;

    #[tokio::test]
    async fn check_database_connectivity() {
        let pool = create_connection_pool().await;

        assert_eq!(pool.is_closed(), false)
    }

    #[tokio::test]
    async fn check_delete_article_success() {
        let pool = create_connection_pool().await;

        let user_id = 1;
        let new_article = Article::new("삭제 제목".to_string(), "삭제 내용".to_string(), user_id);
        let inserted_id = save_article(&pool, new_article).await.unwrap();

        let result = delete_article(&pool, user_id, inserted_id).await;
        result.map_err(|e| println!("{:?}", e)).unwrap();

        // 찾았을 때 Not Found 에러 발생
        assert!(get_detail(&pool, inserted_id).await.is_err());
    }

    #[tokio::test]
    async fn check_not_found() {
        let pool = create_connection_pool().await;

        let no_id = -32;
        let user_id = 1;

        let result = delete_article(&pool, user_id, no_id).await;

        assert!(result.is_err());

        let err_type = match *result.err().unwrap() {
            CustomError::NotFound(_) => true,
            _ => false,
        };
        assert!(err_type)
    }

    #[tokio::test]
    async fn check_no_authority() {
        let pool = create_connection_pool().await;

        let user_id = 1;
        let new_article = Article::new(
            "삭제되지 않는 제목".to_string(),
            "삭제되지 않는 내용".to_string(),
            user_id,
        );

        let article_id = save_article(&pool, new_article).await.unwrap();
        let no_auth_user_id = 2;

        let result = delete_article(&pool, no_auth_user_id, article_id).await;

        assert!(result.is_err());

        let err_type = match *result.err().unwrap() {
            CustomError::Unauthorized(_) => true,
            _ => false,
        };
        assert!(err_type)
    }
}
