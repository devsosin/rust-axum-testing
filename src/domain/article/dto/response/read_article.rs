use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReadArticleResponse {
    id: i64,
    title: String,
    content: String,
    writer_id: i64,
}

impl ReadArticleResponse {
    pub fn new(id: i64, title: String, content: String, writer_id: i64) -> Self {
        Self {
            id,
            title,
            content,
            writer_id,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
}
