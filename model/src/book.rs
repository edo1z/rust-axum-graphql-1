use super::ArcStorage;
use super::Author;
use async_graphql::Context;
use async_graphql::{Object, ID};

#[derive(Clone)]
pub struct Book {
    pub id: ID,
    pub title: String,
    pub author_id: Option<ID>,
}
#[Object]
impl Book {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn title(&self) -> String {
        self.title.clone()
    }
    pub async fn author(&self, ctx: &Context<'_>) -> Option<Author> {
        let storage = ctx.data_unchecked::<ArcStorage>().lock().unwrap();
        match self.author_id.clone() {
            Some(author_id) => storage.author.find(author_id.to_string()),
            None => None,
        }
    }
}
