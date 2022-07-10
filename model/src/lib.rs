pub mod dummy_data;

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn books(&self) -> Vec<Book> {
        dummy_data::books()
    }
    pub async fn book(&self) -> Book {
        Book {
            title: "book1".to_string(),
        }
    }
}

pub struct Book {
    pub title: String,
}
#[Object]
impl Book {
    pub async fn title(&self) -> String {
        self.title.clone()
    }
}
