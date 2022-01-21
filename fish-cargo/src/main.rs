extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11); // 1 - 10
    println!("Random Number: {}", random_number);

    // Flip a coin
    // let coin = rand::thread_rng().gen_weighted_bool(2); // 50% chance of true
    // println!("Coin: {}", coin);
}
