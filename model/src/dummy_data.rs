use super::Book;

pub fn books() -> Vec<Book> {
    vec![
        Book {
            title: String::from("book1"),
        },
        Book {
            title: String::from("book2"),
        },
        Book {
            title: String::from("book3"),
        },
    ]
}
