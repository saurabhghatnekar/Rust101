mod library;

use crate::library::{Book, BookAvailability};

mod person;


/// '&' is used to borrow data. When data is borrowed the ownership is not transferred

fn main() {
    let person = person::Person { name: "Billy".to_owned(), age: 21 };
    person::print_person_info(&person);
    println!();

    let best_book = Book {
        title: "Designing Data-Intensive Applications".to_owned(),
        author: "Martin Kleppmann".to_owned(),
        is_available: BookAvailability::Available,
    };
    let book2 = Book {
        title: "The Rust Programming Language".to_owned(),
        author: "Steve Klabnik and Carol Nichols".to_owned(),
        is_available: BookAvailability::Available,
    };

    let book3 = Book {
        title: "The Clean Coder".to_owned(),
        author: "Robert Cecil Martin".to_owned(),
        is_available: BookAvailability::Available,
    };

    let mut library = library::Library {
        books: vec![]
    };

    library.books.push(best_book);
    library.books.push(book2);
    library.books.push(book3);

    println!("Available books:");
    library.list_available_books();
    println!();

    library.borrow_book(0, "Saurabh");

    println!("Borrowed books:");
    library.list_borrowed_books();
    println!();

    library.return_book(1);
    library.return_book(0);

    println!("Available books:");
    library.list_available_books();
}
