fn main() {
    let mut x = 10;

    {
        let x = 15;

        // do stuff with 15
    }

    let x = "X is a string";
    println!("{}", x);

    let x = true;
    println!("{}", x);
}
