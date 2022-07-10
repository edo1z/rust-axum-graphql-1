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
