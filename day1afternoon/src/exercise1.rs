// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: vec![],
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        // self自身の引数は呼び出しの時はいらないので注意
        if self.len() == 0{
            true
        } else {
            false
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for (idx, book) in (self.books).iter().enumerate() {
            println!("num: {}", idx);
            println!("Title: {}", book.title);
            println!("Year: {}", book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.len() > 0 {
            // 返す値が参照したborrowingのBookなので、&をつける
            Some(&self.books[0])
        } else {
            None
        }
    }
}

fn main() {
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());
    //
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    library.print_books();
    //
    match library.oldest_book() {
        Some(first_book) => println!("My oldest book is {first_book}"),
        None => println!("My library is empty!"),
    }
    //
    println!("Our library has {} books", library.len());
}