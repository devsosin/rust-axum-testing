use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use sqlx::PgPool;

use super::{
    handler::{
        create::create_article, delete::delete_article, read_article::read_article,
        update::update_article,
    },
    repository::ArticleRepositoryImpl,
    usecase::ArticleUsecaseImpl,
};

pub fn get_router(pool: &Arc<PgPool>) -> Router {
    let repository = ArticleRepositoryImpl::new(&pool);
    let usecase = ArticleUsecaseImpl::new(Arc::new(repository));

    Router::new().layer(Extension(Arc::new(usecase)))
}
