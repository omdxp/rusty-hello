use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your name:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Hello, {}", input.trim()),
        Err(error) => println!("Error: {}", error),
    }
}
