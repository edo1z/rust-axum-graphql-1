pub mod dummy_data;

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, ID};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn books(&self) -> Vec<Book> {
        dummy_data::books()
    }
    pub async fn book(&self) -> Book {
        Book {
            id: "0".into(),
            title: "book1".to_string(),
        }
    }
}

pub struct Book {
    pub id: ID,
    pub title: String,
}
#[Object]
impl Book {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn title(&self) -> String {
        self.title.clone()
    }
}
