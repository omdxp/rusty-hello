fn main() {
    let tup1 = (20, "Rust", 30, 3.4, false, (1, 4, 7));

    println!("{}", tup1.2);
    println!("{}", (tup1.5).2);

    let (a, b, c, d, e, f) = tup1;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}
