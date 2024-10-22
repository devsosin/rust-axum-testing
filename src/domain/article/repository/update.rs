use std::sync::Arc;

use sqlx::PgPool;

use crate::global::errors::CustomError;

#[derive(Debug, sqlx::FromRow)]
struct UpdateResult {
    is_exist: bool,
    is_authorized: bool,
    update_count: i64,
}

fn make_query(index: &mut i32, field_name: &str) -> String {
    *index += 1;
    format!("{} = ${}, ", field_name, index)
}

pub async fn update_article(
    pool: &PgPool,
    user_id: i64,
    article_id: i64,
    title: Option<String>,
    content: Option<String>,
) -> Result<(), Arc<CustomError>> {
    let mut query = r#"
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
    "#
    .to_string();

    let mut index = 2;

    let mut update_query = r#"
        UpdateArticle AS (
            UPDATE tb_article SET "#
        .to_string();

    if let Some(_) = title {
        update_query.push_str(&make_query(&mut index, "title"));
    }
    if let Some(_) = content {
        update_query.push_str(&make_query(&mut index, "content"));
    }

    update_query.pop();
    update_query.pop();

    update_query.push_str(
        "
    WHERE id = $1 
        AND (SELECT is_authorized FROM AuthorityChecks) = true
    RETURNING id
    )",
    );
    query.push_str(
        &(update_query
            + r#"
    SELECT
        EXISTS (SELECT 1 FROM ArticleExists) AS is_exist,
        (SELECT is_authorized FROM AuthorityChecks) AS is_authorized,
        (SELECT COUNT(*) FROM UpdateArticle) AS update_count;
    "#),
    );

    let mut query_builder = sqlx::query_as::<_, UpdateResult>(&query)
        .bind(article_id)
        .bind(user_id);

    if let Some(v) = title {
        query_builder = query_builder.bind(v);
    }
    if let Some(v) = content {
        query_builder = query_builder.bind(v);
    }

    let result = query_builder.fetch_one(pool).await.map_err(|e| {
        let err_msg = format!("Error(UpdateArticle): {:?}", e);
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

    use super::update_article;

    #[tokio::test]
    async fn check_database_connectivity() {
        // Arrange, Act
        let pool = create_connection_pool().await;

        assert_eq!(pool.is_closed(), false)
    }

    #[tokio::test]
    async fn check_update_article_success() {
        // Arrange
        let pool = create_connection_pool().await;
        let user_id = 1;
        let new_article = Article::new(
            "바뀌어야 하는 제목".to_string(),
            "바뀌어야 하는 내용".to_string(),
            user_id,
        );

        let article_id = save_article(&pool, new_article.clone()).await.unwrap();

        // Act
        let result = update_article(
            &pool,
            user_id,
            article_id,
            Some("바꿀 제목".to_string()),
            Some("바꿀 내용".to_string()),
        )
        .await;
        result.map_err(|e| println!("{:?}", e)).unwrap();

        // Assert
        let article = get_detail(&pool, article_id).await.unwrap();
        // 달라야 함
        assert_ne!(article.get_title(), new_article.get_title());
        assert_ne!(article.get_content(), new_article.get_content());
    }

    #[tokio::test]
    async fn check_update_only_title() {
        // Arrange
        let pool = create_connection_pool().await;
        let user_id = 1;
        let new_article = Article::new(
            "바뀌어야 하는 제목".to_string(),
            "안바뀌는 내용".to_string(),
            user_id,
        );

        let article_id = save_article(&pool, new_article.clone()).await.unwrap();

        // Act
        let result = update_article(
            &pool,
            user_id,
            article_id,
            Some("바꿀 제목".to_string()),
            None,
        )
        .await;
        result.map_err(|e| println!("{:?}", e)).unwrap();

        // Assert
        let article = get_detail(&pool, article_id).await.unwrap();
        // 달라야 함
        assert_ne!(article.get_title(), new_article.get_title());
        // 내용은 같아야함
        assert_eq!(article.get_content(), new_article.get_content());
    }

    #[tokio::test]
    async fn check_article_not_found() {
        // Arrange
        let pool = create_connection_pool().await;
        let no_id = -32;

        // Act
        let result = update_article(
            &pool,
            1,
            no_id,
            Some("없는 제목".to_string()),
            Some("없는 내용".to_string()),
        )
        .await;

        // Assert
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
            "바뀌면 안되는 제목".to_string(),
            "바뀌면 안되는 내용".to_string(),
            user_id,
        );

        let article_id = save_article(&pool, new_article.clone()).await.unwrap();
        let no_auth_user_id = 2;

        let result = update_article(
            &pool,
            no_auth_user_id,
            article_id,
            Some("바뀌지 않음".to_string()),
            Some("바뀌지 않음".to_string()),
        )
        .await;

        assert!(result.is_err());

        let err_type = match *result.err().unwrap() {
            CustomError::Unauthorized(_) => true,
            _ => false,
        };
        assert!(err_type)
    }
}
