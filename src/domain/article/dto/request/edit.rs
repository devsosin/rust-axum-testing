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

    pub fn check_fields(&self) -> bool {
        let v1 = self.title.as_ref().is_some() && !self.title.as_ref().unwrap().is_empty();
        let v2 = self.content.as_ref().is_some() && !self.content.as_ref().unwrap().is_empty();

        v1 && v2
    }

    pub fn to_fields(&self) -> (Option<String>, Option<String>) {
        (self.title.to_owned(), self.content.to_owned())
    }
}
