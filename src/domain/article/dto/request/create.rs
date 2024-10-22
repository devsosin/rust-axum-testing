use serde::{Deserialize, Serialize};

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
}
