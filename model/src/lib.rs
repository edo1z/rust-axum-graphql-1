pub mod author;
pub mod book;
pub mod dummy_data;

use async_graphql::{Context, EmptySubscription, Object, Schema};
use author::Author;
use book::Book;
use std::sync::{Arc, Mutex};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub type ArcStorage = Arc<Mutex<dummy_data::Storage>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.book.list().clone()
    }
    pub async fn book(&self, ctx: &Context<'_>, id: String) -> Option<Book> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.book.find(id)
    }

    pub async fn authors(&self, ctx: &Context<'_>) -> Vec<Author> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.author.list().clone()
    }

    pub async fn author(&self, ctx: &Context<'_>, id: String) -> Option<Author> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.author.find(id)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn add_book(
        &self,
        ctx: &Context<'_>,
        title: String,
        author_id: Option<String>,
    ) -> Result<Book, String> {
        let mut storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.add_book(title, author_id)
    }

    pub async fn add_author(&self, ctx: &Context<'_>, name: String) -> Result<Author, String> {
        let mut storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.author.add(name)
    }
}
