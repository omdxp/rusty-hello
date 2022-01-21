struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    // speak
    fn speak(&self);
    // check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("{} says hello", self.name);
    }

    fn can_speak(&self) -> bool {
        self.age > 18
    }
}

fn main() {
    let omar = Person {
        name: String::from("Omar"),
        age: 23,
    };
    if omar.can_speak() {
        omar.speak()
    }
}
