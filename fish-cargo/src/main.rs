fn main() {
    let mut x = 10;
    // let xr = &x;

    let o = &mut x;
    *o += 1;

    println!("{}", x);
}
