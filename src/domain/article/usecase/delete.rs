use std::sync::Arc;

use crate::{domain::article::repository::ArticleRepository, global::errors::CustomError};

pub async fn delete_article(
    repository: Arc<dyn ArticleRepository>,
    user_id: i64,
    article_id: i64,
) -> Result<(), Arc<CustomError>> {
    repository.delete_article(user_id, article_id).await
}
