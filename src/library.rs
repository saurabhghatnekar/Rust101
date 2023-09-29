use crate::library::BookAvailability::Available;

pub(crate) enum BookAvailability {
    Available,
    Borrowed(String),
}

pub(crate) struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) is_available: BookAvailability,
}

// impl Book {
//     pub(crate) fn borrow_book(&mut self, borrower: &str) -> bool {
//         match self.is_available {
//             Available => {
//                 BookAvailability::Borrowed(borrower.to_owned());
//                 true
//             }
//             BookAvailability::Borrowed(_) => {false}
//         }
//     }
//
//     pub(crate) fn return_book(&mut self) ->bool {
//         match self.is_available {
//             Available => { false }
//             BookAvailability::Borrowed(_) => {
//                 self.is_available = BookAvailability::Available;
//                 true
//             }
//         }
//     }
// }

pub(crate) struct Library {
    pub(crate) books: Vec<Book>,
}

impl Library {
    pub(crate) fn list_borrowed_books(&self) {
        for book in self.books.iter() {
            match &book.is_available {
                Available => {}
                BookAvailability::Borrowed(name) => {
                    println!("{} is borrowed by {}", book.title, name)
                }
            }
        }
    }

    pub(crate) fn list_available_books(&self) {
        for book in self.books.iter() {
            match &book.is_available {
                Available => { println!("{}", &book.title) }
                BookAvailability::Borrowed(name) => {}
            }
        }
    }

    pub(crate) fn borrow_book(&mut self, index: usize, borrower: &str) -> bool {
        let book = &mut self.books[index];

        match book.is_available {
            Available => {
                book.is_available = BookAvailability::Borrowed(borrower.to_owned());
                true
            }
            BookAvailability::Borrowed(_) => { false }
        }
    }

    pub(crate) fn return_book(&mut self, index: usize) -> bool {
        let mut book = &mut self.books[index];

        match book.is_available {
            Available => {
                println!("You cannot return this book");
                false
            }
            BookAvailability::Borrowed(_) => {
                book.is_available = Available;
                true
            }
        }
    }
}