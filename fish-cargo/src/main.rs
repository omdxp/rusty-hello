fn main() {
    // Replace
    {
        let my_string = String::from("Rust is fantastic!");
        println!(
            "After replace: {}",
            my_string.replace("fantastic", "awesome")
        );
    }

    // Lines
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");

        for line in my_string.lines() {
            println!("[ {} ]", line)
        }
    }

    // Split
    {
        let my_string = String::from("Leave+a+fish+in+the+water");

        let tokens: Vec<&str> = my_string.split('+').collect();

        for token in tokens {
            println!("[ {} ]", token)
        }
    }

    // Trim
    {
        let my_string = String::from("  Remove whitespace  ");

        println!("[ {} ]", my_string.trim());
    }

    // Chars
    {
        let my_string = String::from("Fish are awesome!");

        // Get the character at index
        match my_string.chars().nth(2) {
            Some(c) => println!("[ {} ]", c),
            None => println!("[ None ]"),
        }
    }
}
