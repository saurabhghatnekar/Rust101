struct Person {
    name: String,
    age: usize,
}

struct Book {
    title: String,
    author: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}

fn main() {
    let person = Person { name: "Billy".to_owned(), age: 21 };

    let best_book = Book {
        title: "Designing Data-Intensive Applications".to_owned(),
        author: "Martin Kleppmann".to_owned(),
        is_available: true,
    };

    let mut library = Library {
        books: vec![]
    };

    library.books.push(best_book);

}
