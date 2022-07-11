pub mod author;
pub mod book;
pub mod dummy_data;

use async_graphql::{EmptySubscription, Object, Schema};
use author::Author;
use book::Book;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn books(&self) -> Vec<Book> {
        dummy_data::books()
    }
    pub async fn book(&self, id: String) -> Option<Book> {
        let books = dummy_data::books();
        let result = books.iter().find(|&x| x.id.to_string() == id);
        result.cloned()
    }

    pub async fn authors(&self) -> Vec<Author> {
        dummy_data::authors()
    }

    pub async fn author(&self, id: String) -> Option<Author> {
        let authors = dummy_data::authors();
        let result = authors.iter().find(|&x| x.id.to_string() == id);
        result.cloned()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn add_book(&self, title: String, author_id: Option<String>) -> Result<Book, String> {
        Err(String::from("ERROR! failed add book"))
    }
}
