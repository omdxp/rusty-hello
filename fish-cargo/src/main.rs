fn main() {
    let mut n = 1;

    while n <= 50 {
        if n % 5 == 0 {
            println!("{}", n);
        }
        n += 1;
    }
}
