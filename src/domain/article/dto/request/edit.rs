use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct EditArticleRequest {
    title: Option<String>,
    content: Option<String>,
}

impl EditArticleRequest {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        Self { title, content }
    }

    pub fn to_fields(&self) -> (Option<String>, Option<String>) {
        (self.title.to_owned(), self.content.to_owned())
    }
}
