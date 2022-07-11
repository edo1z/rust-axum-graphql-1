use super::Author;
use super::Book;
use uuid::Uuid;

pub struct Storage {
    pub book: BookData,
    pub author: AuthorData,
}
impl Storage {
    pub fn new() -> Self {
        Self {
            book: BookData::new(),
            author: AuthorData::new(),
        }
    }
}

pub struct BookData {
    pub data: Vec<Book>,
}
impl BookData {
    pub fn new() -> Self {
        Self {
            data: vec![
                Book {
                    id: "1".into(),
                    title: String::from("book1"),
                },
                Book {
                    id: "2".into(),
                    title: String::from("book2"),
                },
                Book {
                    id: "3".into(),
                    title: String::from("book3"),
                },
            ],
        }
    }
    pub fn list(&self) -> &Vec<Book> {
        &self.data
    }
    pub fn find(&self, id: String) -> Option<Book> {
        let result = self.data.iter().find(|&x| x.id.to_string() == id);
        result.cloned()
    }
    pub fn add(&mut self, title: String) -> Result<Book, String> {
        let new_book = Book {
            id: Uuid::new_v4().into(),
            title,
        };
        self.data.push(new_book.clone());
        Ok(new_book)
    }
}

pub struct AuthorData {
    pub data: Vec<Author>,
}
impl AuthorData {
    pub fn new() -> Self {
        Self {
            data: vec![
                Author {
                    id: "1".into(),
                    name: String::from("taro"),
                },
                Author {
                    id: "2".into(),
                    name: String::from("jiro"),
                },
                Author {
                    id: "3".into(),
                    name: String::from("saburo"),
                },
            ],
        }
    }
    pub fn list(&self) -> &Vec<Author> {
        &self.data
    }
    pub fn find(&self, id: String) -> Option<Author> {
        let result = self.data.iter().find(|&x| x.id.to_string() == id);
        result.cloned()
    }

    pub fn add(&mut self, name: String) -> Result<Author, String> {
        let new_author = Author {
            id: Uuid::new_v4().into(),
            name,
        };
        self.data.push(new_author.clone());
        Ok(new_author)
    }
}
