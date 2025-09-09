
#  CLI with Clap in Rust

## Why use Clap?

In Rust, you often want to build **command-line tools** that accept arguments (instead of hardcoding values).
Manually handling arguments with `std::env::args()` is possible, but **Clap** (Command Line Argument Parser) makes it:

* ✅ Easier to define commands & flags
* ✅ Automatically generates `--help` and usage instructions
* ✅ Provides input validation (numbers, strings, required args, defaults, etc.)
* ✅ Widely used in real-world Rust tools (cargo itself uses Clap!)

---

## Example Project: Simple CLI Calculator

### 1. Create project

```bash
cargo new cli_calculator
cd cli_calculator
```

### 2. Add Clap dependency

In your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.5.4", features = ["derive"] }
```

### 3. Write Code (`src/main.rs`)

```rust
use clap::{Arg, Command};

fn main() {
    // Define CLI structure
    let matches = Command::new("CLI Calculator")
        .version("1.0")
        .author("Sheeza Alam")
        .about("Performs simple arithmetic using Clap")
        .arg(
            Arg::new("operation")
                .help("The operation to perform: add or sub")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("x")
                .help("First number")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("y")
                .help("Second number")
                .required(true)
                .index(3),
        )
        .get_matches();

    // Extract arguments
    let operation = matches.get_one::<String>("operation").unwrap();
    let x: i32 = matches.get_one::<String>("x").unwrap().parse().expect("Invalid number");
    let y: i32 = matches.get_one::<String>("y").unwrap().parse().expect("Invalid number");

    // Perform operation
    let result = match operation.as_str() {
        "add" => x + y,
        "sub" => x - y,
        _ => {
            eprintln!("Unknown operation: {}", operation);
            return;
        }
    };

    println!("Result: {}", result);
}
```

---

### 4. Run Examples

```bash
cargo run -- add 5 3
```

Output:

```
Result: 8
```

```bash
cargo run -- sub 10 7
```

Output:

```
Result: 3
```

```bash
cargo run -- --help
```

Output:

```
CLI Calculator 1.0
Performs simple arithmetic using Clap

USAGE:
    cli_calculator <operation> <x> <y>

ARGS:
    <operation>    The operation to perform: add or sub
    <x>            First number
    <y>            Second number
```

---

## How to Organize Your Repo

In your GitHub `rust-learning-journey`, you can add a folder like this:

```
rust-learning-journey/
├── Functions_Expressions_Statements/
├── Variables_Types/
├── cli_projects/
│   └── cli_calculator/
│       ├── Cargo.toml
│       └── src/main.rs
```

