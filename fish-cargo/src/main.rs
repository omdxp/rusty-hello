fn main() {
    // here
    let x = 10;
    {
        // isolated has access to outside data
        let y = 5;

        println!("x: {} y: {}", x, y)
    }
}
