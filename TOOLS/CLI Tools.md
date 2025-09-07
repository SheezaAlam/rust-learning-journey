Rust CLI Tools (Basics)

cargo new project_name
Creates a new Rust project.

cargo build
Compiles the project and generates an executable in the target/debug directory.

cargo check
Checks code for errors without creating an executable. Faster than build.

cargo run
Compiles (if needed) and runs the project.

Cargo.toml
Project configuration file (stores metadata and dependencies).

Cargo.lock
Automatically generated file that records the exact versions of dependencies used.

rustfmt src/main.rs
Formats your Rust code according to official style guidelines.
