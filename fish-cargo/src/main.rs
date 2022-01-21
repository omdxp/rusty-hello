enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Directions = Directions::Down;

    match player_direction {
        Directions::Up => println!("Going up!"),
        Directions::Down => println!("Going down!"),
        Directions::Left => println!("Going left!"),
        Directions::Right => println!("Going right!"),
    }
}
