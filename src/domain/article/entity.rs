use super::dto::response::read_article::ReadArticleResponse;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq)]
pub struct Article {
    id: Option<i64>,
    title: String,
    content: String,
    writer_id: i64,
}

impl Article {
    pub fn new(title: String, content: String, writer_id: i64) -> Self {
        Self {
            id: None,
            title,
            content,
            writer_id,
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);

        self
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_writer(&self) -> i64 {
        self.writer_id
    }

    pub fn to_response(&self) -> ReadArticleResponse {
        ReadArticleResponse::new(
            self.id.unwrap(),
            self.title.to_string(),
            self.content.to_string(),
            self.writer_id,
        )
    }
}
