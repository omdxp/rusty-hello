fn main() {
    println!(
        "Omar is {}",
        match get_occupation("Omar") {
            Some(o) => o,
            None => "a fish",
        }
    );
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Omar" => Some("Sr. Fullstack Engineer"),
        "Jenny" => Some("Chiropractor"),
        _ => None,
    }
}
