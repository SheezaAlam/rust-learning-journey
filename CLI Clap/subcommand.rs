use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { x: i32, y: i32 },
    Sub { x: i32, y: i32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { x, y } => println!("Result = {}", x + y),
        Commands::Sub { x, y } => println!("Result = {}", x - y),
    }
}
/* Output:
PS C:\Users\shiza\tokio_playground> cargo run -- add 5 3                      
   Compiling tokio_playground v0.1.0 (C:\Users\shiza\tokio_playground)        
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.94s       
     Running `target\debug\tokio_playground.exe add 5 3`
Result = 8
 */
