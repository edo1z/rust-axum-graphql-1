use super::Author;
use super::Book;

pub fn books() -> Vec<Book> {
    vec![
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
    ]
}

pub fn authors() -> Vec<Author> {
    vec![
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
    ]
}
