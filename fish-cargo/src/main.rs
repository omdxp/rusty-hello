extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w{5})").unwrap(); // 5-character word
    let text = "omarrr";
    println!("{}", re.is_match(text));

    match re.captures(text) {
        Some(caps) => println!("{}", caps.get(0).unwrap().as_str()),
        None => println!("No match"),
    }
}
