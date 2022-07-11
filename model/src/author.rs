use super::ArcStorage;
use super::Book;
use async_graphql::{Context, Object, ID};

#[derive(Clone)]
pub struct Author {
    pub id: ID,
    pub name: String,
    pub books: Vec<ID>,
}
#[Object]
impl Author {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self) -> String {
        self.name.clone()
    }
    pub async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        storage.book.find_all_by_ids(self.books.clone())
    }
}
