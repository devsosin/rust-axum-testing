pub mod entity;
pub mod route;

pub mod repository;
pub mod usecase;
mod handler {
    pub mod create;
    pub mod delete;
    pub mod read_article;
    pub mod update;
}

pub mod dto {
    pub mod request {
        pub mod create;
        pub mod edit;
    }
    pub mod response {
        pub mod create;
        pub mod read_article;
    }
}
