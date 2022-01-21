fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n == 7 {
            continue;
        }
        println!("{}", n);
        if n == 10 {
            break;
        }
    }
}
