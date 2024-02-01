use std::fs::File;

// use option when a value could be something or nothing
fn find_word(word: &str, text: &str) -> Option<usize> {
    text.find(word)
}

// use result could return error
fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}

fn main(){
    //using unwrap (not recommended)
    let word_position = find_word("hello", "hello world").unwrap();
    println!("The word is at index {}", word_position);

    // handling errors more gracefully
    match find_word("hello", "hello world") {
        Some(position) => println!("The word is at index {}", position),
        None => println!("The word is not found")
    }

    // handling error with Result
    match open_file("non-existent.txt") {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Failed to open the file: {}", error)
    }
}