fn main() {
    print_numbers_to(10);
}

fn print_numbers_to(n: u8) {
    for i in 0..n {
        if is_even(i) {
            println!("{} is even", i)
        } else {
            println!("{} is odd", i)
        }
    }
}

fn is_even(n: u8) -> bool {
    return n % 2 == 0;
}
