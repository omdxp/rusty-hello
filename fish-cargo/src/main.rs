use std::collections::HashMap;

fn main() {
    let mut fish = HashMap::new();
    fish.insert(String::from("Blue"), 10);
    fish.insert(String::from("Yellow"), 50);

    println!("{:?}", fish);

    // find length of HashMap
    println!("{}", fish.len());

    // get value from HashMap
    match fish.get("Blue") {
        Some(v) => println!("{}", v),
        None => println!("Not found"),
    }

    // update value in HashMap
    fish.insert(String::from("Blue"), 25);
    println!("{:?}", fish);

    // remove value from HashMap
    fish.remove("Blue");
    println!("{:?}", fish);

    // iterate over HashMap
    for (key, value) in &fish {
        println!("{}: {}", key, value);
    }

    // check if key exists in HashMap
    if fish.contains_key("Yellow") {
        println!("Yellow exists");
    }
}
