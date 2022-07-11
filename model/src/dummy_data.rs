use super::Author;
use super::Book;
use async_graphql::ID;
use uuid::Uuid;

#[derive(Default)]
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
    pub fn add_book(&mut self, title: String, author_id: Option<String>) -> Result<Book, String> {
        let mut new_book = Book {
            id: Uuid::new_v4().into(),
            title,
            author_id: None,
        };
        if let Some(id) = author_id {
            if self.author.find(id.clone()).is_some() {
                new_book.author_id = Some(id.clone().into());
                let _ = self.author.add_book(id, new_book.id.clone());
            }
        }
        self.book.add(new_book)
    }
}

#[derive(Default)]
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
                    author_id: None,
                },
                Book {
                    id: "2".into(),
                    title: String::from("book2"),
                    author_id: None,
                },
                Book {
                    id: "3".into(),
                    title: String::from("book3"),
                    author_id: None,
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
    pub fn add(&mut self, new_book: Book) -> Result<Book, String> {
        self.data.push(new_book.clone());
        Ok(new_book)
    }
}

#[derive(Default)]
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
                    books: vec![],
                },
                Author {
                    id: "2".into(),
                    name: String::from("jiro"),
                    books: vec![],
                },
                Author {
                    id: "3".into(),
                    name: String::from("saburo"),
                    books: vec![],
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
    pub fn add_book(&mut self, id: String, book_id: ID) -> Result<String, String> {
        let result = self
            .data
            .iter()
            .position(|x| x.id.clone().to_string() == id);
        match result {
            None => Err(String::from("ERROR! Book is not found.")),
            Some(idx) => {
                self.data[idx].books.push(book_id);
                Ok(String::from(""))
            }
        }
    }
    pub fn add(&mut self, name: String) -> Result<Author, String> {
        let new_author = Author {
            id: Uuid::new_v4().into(),
            name,
            books: vec![],
        };
        self.data.push(new_author.clone());
        Ok(new_author)
    }
}
