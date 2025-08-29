Day 5 – Console Input in Rust

In Rust, to take user input from the console, we use the std::io library.

Example Code
use std::io; // standard library for input/output

fn main() {
    let mut input = String::new();

    // read a line from the console and store in `input`
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("{}", input);
}

Explanation

use std::io; → imports the standard I/O library.

let mut input = String::new(); → creates a mutable empty string to hold the input.

io::stdin().read_line(&mut input) → reads user input and stores it in input.

&mut input → gives a mutable reference so Rust can update the variable.

.expect("failed to read line") → handles possible errors during input.

println!("{}", input); → prints the user’s input back.

Sample Run
> cargo run
hello rust
hello rust

Improvements

If you want to avoid the newline \n being included when you press Enter, you can trim the input:

println!("{}", input.trim());

Progress Tracker

 Day 1: Setup + CLI Tools

 Day 2: Constants

 Day 3: Variables + Shadowing

 Day 4: Data Types

 Day 5: Console Input

 Day 6: Coming soon…

Connect with Me

I’m documenting my Rust Learning Journey from scratch. Follow my updates:

GitHub: rust-learning-journey

Twitter: @Khan_shizaalam

Discord: https://discord.com/channels/1409601307991343199/1409601608194719985
