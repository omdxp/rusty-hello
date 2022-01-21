struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    // color: red, green, blue
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    color.blue = 45;

    println!("{}, {}, {}", color.red, color.green, color.blue)
}
