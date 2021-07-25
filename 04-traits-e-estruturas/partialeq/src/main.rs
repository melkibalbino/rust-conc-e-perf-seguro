enum BookFormat {
    Paperback,
    Hardback,
    Ebook
}

#[allow(dead_code)]
struct Book {
    isbn: i32,
    title: &'static str,
    format: BookFormat
}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {
    let b1 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: BookFormat::Paperback
    };

    let b2 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: BookFormat::Ebook
    };

    let b3 = Book {
        isbn: 1234567810,
        title: "O Hobbit",
        format: BookFormat::Hardback
    };

    println!("{}", b1 == b2);
    println!("{}", b1 == b3);
    println!("{}", b2 == b3)
}