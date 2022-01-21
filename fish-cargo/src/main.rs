fn main() {
    let mut my_string = String::from("How's it going? My name is Fish");

    // length
    println!("Length: {}", my_string.len());
    // Is Empty?
    println!("String is empty? {}", my_string.is_empty());
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("{}", my_string.replace("Fish", "FISH"));
    my_string.push_str(" and I'm a Rustacean!");
    println!("{}", my_string);

    println!("{}", my_string.chars().count());
    println!("{}", my_string.chars().rev().collect::<String>());
    println!(
        "Does the string contain 'Fish'? {}",
        my_string.contains("Fish")
    );
}
