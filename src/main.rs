use std::io::{self, Write};

fn greet(name: &str) {
    println!("\nNice to meet you, {}! 🎉", name);
    println!("Fun fact: The Rust programming language is named after a fungus.");
    println!("Keep building awesome things! 🦀");
}

fn main() {
    println!("Hello, world!");
    println!("Welcome to EntireHQ!\n");

    print!("What's your name? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    if name.is_empty() {
        println!("No name entered — but welcome anyway, mysterious stranger!");
    } else {
        greet(name);
    }
}
