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

impl Book {
    // associate function
    fn new(title: &str, author: &str, pages: u32, price: f32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
            price,
        }
    }

    // method to get string representation of a Book
    fn get_info(&self) -> String {
        format!("{} is written by {} and has {} pages. Price is ${:.2}", self.title, self.author, self.pages, self.price)
    }
}   

fn main(){
    // create a struct instance
    let book = Book::new("The Odyssey", "Homer", 608, 9.99);
    // print the book info
    println!("{}", book.get_info());
}
