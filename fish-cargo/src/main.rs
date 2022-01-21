mod fish {
    fn fish() {
        println!("Fish");
    }

    pub fn print_message() {
        fish();
        println!("How's it going!")
    }

    pub mod day {
        pub fn fish() {
            println!("Day fish");
        }
    }
}

fn main() {
    fish::print_message();
    fish::day::fish()
}
