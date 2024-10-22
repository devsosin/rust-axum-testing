use serde::{Deserialize, Serialize};

use crate::domain::article::entity::Article;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ArticleCreateRequest {
    title: String,
    content: String,
}

impl ArticleCreateRequest {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn to_entity(&self, user_id: i64) -> Article {
        Article::new(self.title.to_string(), self.content.to_string(), user_id)
    }
}
