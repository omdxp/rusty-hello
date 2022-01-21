fn main() {
    let number = 6;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4..=9 => println!("four to nine"),
        10 | 11 => println!("ten or eleven"),
        _ => println!("anything"),
    }

    let name = "Yasser";
    match name {
        "Omar" => println!("Omar"),
        "Yasser" => println!("Yasser"),
        _ => println!("anyone else"),
    }
}
