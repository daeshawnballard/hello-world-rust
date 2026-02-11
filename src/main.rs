use rand::Rng;
use std::io::{self, Write};

fn greet(name: &str) {
    let greetings = [
        "Ahoy there, {}! Welcome aboard the Rust ship! 🚀",
        "Hey hey, {}! You're officially a Rustacean now! 🦀",
        "Well well well, if it isn't {}! Ready to code? 💻",
        "Greetings, {}! May your code compile on the first try! ✨",
        "Yo, {}! Let's build something amazing together! 🔥",
    ];

    let fun_facts = [
        "The Rust programming language is named after a fungus. 🍄",
        "Rust has been the most loved language on Stack Overflow for years running. 💕",
        "Rust's mascot, Ferris, is a crab — not a lobster! 🦀",
        "Rust guarantees memory safety without needing a garbage collector. 🧹",
        "The first version of the Rust compiler was written in OCaml. 🐫",
    ];

    let mut rng = rand::thread_rng();
    let greet_idx = rng.gen_range(0..greetings.len());
    let fact_idx = rng.gen_range(0..fun_facts.len());
    println!("\n{}", greetings[greet_idx].replace("{}", name));
    println!("Fun fact: {}", fun_facts[fact_idx]);
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
