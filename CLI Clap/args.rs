use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "myapp", about = "An example CLI with clap")]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
/*output:
PS C:\Users\shiza\tokio_playground> cargo run -- --name Shiza --count 3       
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s       
     Running `target\debug\tokio_playground.exe --name Shiza --count 3`       
Hello, Shiza!
Hello, Shiza!
Hello, Shiza */
