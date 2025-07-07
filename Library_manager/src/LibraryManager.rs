#[derive(Debug)]  //This is an attribute that tells Rust to automatically generate code so we can print the book with println!("{:?}", book). Very useful for debugging.
struct Book {   ////Here you define a structure called Book, which groups 3 pieces of information:
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
