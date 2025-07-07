#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
}

impl Book {
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
        }
    }
}
