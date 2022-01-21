fn main() {
    for i in 0..10 {
        println!("{}", i);
    }
    let numbers = 30..51;

    for i in numbers {
        println!("{}", i);
    }

    let animals = vec!["cat", "dog", "cow", "fish"];

    for (i, a) in animals.iter().enumerate() {
        println!("{} {}", i, a);
    }
}
