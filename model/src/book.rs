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
}
