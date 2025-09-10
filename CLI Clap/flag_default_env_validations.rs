// =========================
// 1. What are Flags?
// =========================
// Flags are instructions passed from command line.
// Example: 
//   cargo run -- --bind 192.168.1.10 --port 9000
//
// Here:
//   --bind tells which host to bind to
//   --port tells which port number to listen on

/* 
use clap::Parser;
#[derive(Parser,Debug)] // clap auto handles CLI parsing

struct Cli {
    #[arg(long)]
    bind: String,

    #[arg(long)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("server will bind to: {}:{}", cli.bind, cli.port);
}
*/

// Example Run:
// PS> cargo run -- --bind 192.168.1.10 --port 9000
// server will bind to: 192.168.1.10:9000



// =========================
// 2. What are Defaults?
// =========================
// Defaults mean if you don’t provide a flag, the program
// will use a "safe value" automatically.
// Example: default bind = 127.0.0.1, port = 7878.

/*
use clap::Parser;
#[derive(Parser,Debug)]

struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,

    #[arg(long, default_value_t = 7878)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("server will bind to: {}:{}", cli.bind, cli.port);
}
*/

// Example Run:
// PS> cargo run
// server will bind to: 127.0.0.1:7878
//
// PS> cargo run -- --bind 192.168.1.10 --port 9000
// server will bind to: 192.168.1.10:9000



// =========================
// 3. Env Variables
// =========================
// Env vars are like instructions set outside the code.
// Useful in Docker, CI, or production (team doesn’t have 
// to recompile to change configs).
//
// Example:
//   $env:BIND="10.0.0.5"
//   $env:PORT="8081"
//   cargo run
//
// Output:
//   Server binding to: 10.0.0.5:8081



// =========================
// 4. Validation
// =========================
// Validation ensures inputs are "sane".
// For ports → must be between 0 and 65535.
// Example: if user tries --port 70000 → error instead of crash.



// =========================
// 5. Config Struct (Clean)
// =========================
// This combines: Flags + Defaults + Env Vars + Validation
// into one clean struct.

use clap::Parser;

/// Echo Server Config with validation
#[derive(Parser, Debug)]
struct Cli {
    /// The IP to bind to
    #[arg(long, env = "BIND", default_value = "127.0.0.1")]
    bind: String,

    /// The port to listen on (must be 0–65535)
    #[arg(
        long, 
        env = "PORT", 
        default_value_t = 7878, 
        value_parser = clap::value_parser!(u16).range(0..=65535)
    )]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("Server binding to: {}:{}", cli.bind, cli.port);
}

// Example Run (with invalid port):
// PS> cargo run -- --port 70000
// error: invalid value '70000' for '--port <PORT>': 70000 is not in 0..=65535
//
// Example Run (with env vars):
// PS> $env:BIND="10.0.0.5"
// PS> $env:PORT="8081"
// PS> cargo run
// Server binding to: 10.0.0.5:8081
