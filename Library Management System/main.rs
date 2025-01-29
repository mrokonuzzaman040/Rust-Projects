use std::fmt;
use std::io::{self, Write};

struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nBorrowed: {}\n",
            self.title,
            self.author,
            if self.is_borrowed { "Yes" } else { "No" }
        )
    }
}

impl Book {
    fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            is_borrowed: false,
        }
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn borrow_book(&mut self, title: &str) {
        for book in self.books.iter_mut() {
            if book.title == title {
                if book.is_borrowed {
                    println!("Error: \"{}\" is already borrowed!", title);
                    return;
                } else {
                    book.is_borrowed = true;
                    println!("You have borrowed \"{}\" successfully!", title);
                    return;
                }
            }
        }
        println!("Error: Book \"{}\" not found in the library!", title);
    }

    fn return_book(&mut self, title: &str) {
        for book in self.books.iter_mut() {
            if book.title == title {
                if book.is_borrowed {
                    book.is_borrowed = false;
                    println!("You have returned \"{}\" successfully!", title);
                    return;
                } else {
                    println!("Error: \"{}\" was not borrowed!", title);
                    return;
                }
            }
        }
        println!("Error: Book \"{}\" not found in the library!", title);
    }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("The library has no books.");
            return;
        }

        println!("\nAvailable Books:");
        for (index, book) in self.books.iter().enumerate() {
            println!("{}. {}", index + 1, book);
        }
    }
}

fn main() {
    let mut library = Library::new();

    library.add_book(Book::new(
        "The Catcher in the Rye".to_string(),
        "J.D. Salinger".to_string(),
    ));
    library.add_book(Book::new("1984".to_string(), "George Orwell".to_string()));
    library.add_book(Book::new(
        "To Kill a Mockingbird".to_string(),
        "Harper Lee".to_string(),
    ));

    loop {
        println!("\nLibrary Management System:");
        println!("1. List books");
        println!("2. Borrow book");
        println!("3. Return book");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 4.");
                continue;
            }
        };

        match choice {
            1 => library.list_books(),
            2 => {
                print!("Enter the title of the book you want to borrow: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();
                library.borrow_book(title);
            }
            3 => {
                print!("Enter the title of the book you want to return: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();
                library.return_book(title);
            }
            4 => {
                println!("Exiting Library Management System. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 4."),
        }
    }
}
