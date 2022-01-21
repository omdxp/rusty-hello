#![allow(dead_code)]

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekend(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => true,
            _ => false,
        }
    }
}

fn main() {
    let day = Day::Sunday;
    println!("{}", day.is_weekend());
}
