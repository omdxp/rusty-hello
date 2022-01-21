struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let omar = Person {
        name: String::from("Omar"),
        age: 23,
    };
    println!("{}", omar.to_string());
}
