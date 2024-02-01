// deinfe a struct
struct Book {
    title: String,
    author: String,
    pages: u32,
    price: f32,
}

//define an enum
enum Status {
    Available,
    Sold,
    Borrowed
}

impl Status {
    // method to get string representation of a Status
    fn as_str(&self) -> &'static str {
        match *self {
            Status::Available => "available",
            Status::Sold => "sold",
            Status::Borrowed => "borrowed",
        }
    }
}

impl Book {
    // associate function
    fn new(title: &str, author: &str, pages: u32, price: f32, status: Status) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
            price,
        }
    }

    // method to get string representation of a Book
    fn get_info(&self) -> String {
        format!("{} is written by {} and has {} pages. Price is ${:.2}. Status: {}", self.title, self.author, self.pages, self.price, self.status.as_str())
    }
}   

fn main(){
    // create a struct instance
    let book = Book::new("The Odyssey", "Homer", 608, 9.99, Status::Available);
    // print the book info
    println!("{}", book.get_info());
}
