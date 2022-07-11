use async_graphql::{Object, ID};

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
}
