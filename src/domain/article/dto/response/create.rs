use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ArticleCreateResponse {
    id: i64,
}

impl ArticleCreateResponse {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
}
