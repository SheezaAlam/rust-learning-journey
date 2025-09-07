## Repository Structure

rust-structs-teaching/
├── README.md
├── Cargo.toml
├── LICENSE
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── examples/
│       ├── basic_struct.rs
│       ├── tuple_struct.rs
│       ├── unit_struct.rs
│       ├── methods_and_impl.rs
│       ├── ownership_and_borrowing.rs
│       └── derive_debug_clone.rs
├── examples/              # runnable `cargo run --example <name>` examples
│   ├── basic_struct.rs
│   └── methods_and_impl.rs
├── tests/
│   └── struct_tests.rs
└── CONTRIBUTING.md

# Rust Structs — Teaching Examples

This repository walks through Rust's `struct` types with progressive examples and exercises:

- Basic named structs
- Tuple structs
- Unit-like structs
- `impl` blocks: associated functions and methods
- Mutability, ownership, and borrowing with structs
- Deriving traits (`Debug`, `Clone`, `PartialEq`, etc.)

## How to use this repository

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository
3. Open the project folder and run examples with:

```bash
cargo run --bin basic_struct
cargo run --example methods_and_impl
````

Run the test suite:

```bash
cargo test
```

## Learning goals

* Understand differences between named, tuple, and unit structs.
* Know how to attach behavior using `impl` blocks.
* Learn how ownership and borrowing interact with struct fields.
* Practice writing idiomatic struct code and tests.

## Exercises

1. Implement a `Point` struct with `x` and `y` fields and add a method that computes distance to another point.
2. Create a `Rectangle` struct with `width` and `height` and implement an `area` method.
3. Implement `From<(i32, i32)>` for `Point` and write tests.

## License

This repository is provided under the MIT license.

```
```

---

## Cargo.toml (example)

```toml
[package]
name = "rust-structs-teaching"
version = "0.1.0"
edition = "2021"

[dependencies]
# no external dependencies required for basic examples

[dev-dependencies]
# dev/test helpers if needed
```

---

## src/main.rs

```rust
fn main() {
    println!("Run `cargo run --example basic_struct` or see src/examples for examples.");
}
```

---

## src/lib.rs

This library file exposes a few reusable structs and functions useful in examples and tests.

```rust
/// A basic 2D point using named fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Associated function: constructor
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Compute squared distance to another point
    pub fn distance_squared(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Compute Euclidean distance to another point
    pub fn distance(&self, other: &Self) -> f64 {
        self.distance_squared(other).sqrt()
    }
}

/// A rectangle with width and height.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

---

## src/examples/basic\_struct.rs

```rust
use rust_structs_teaching::Point;

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
    println!("Distance = {}", p1.distance(&p2));
}
```

---

## src/examples/tuple\_struct.rs

```rust
// Tuple structs are like tuples but carry a name.
#[derive(Debug, Clone, PartialEq)]
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    println!("black = {:?}, white = {:?}", black, white);
}
```

---

## src/examples/unit\_struct.rs

```rust
// Unit-like structs carry no data. They are useful for marker types.
#[derive(Debug, Clone, PartialEq)]
struct Marker;

fn consume_marker(_m: Marker) {
    println!("Marker consumed");
}

fn main() {
    let m = Marker;
    consume_marker(m);
}
```

---

## src/examples/methods\_and\_impl.rs

```rust
use rust_structs_teaching::{Rectangle, Point};

fn main() {
    let rect = Rectangle::new(10, 5);
    println!("Rectangle area = {}", rect.area());

    let a = Point::new(1.0, 1.0);
    let b = Point::new(4.0, 5.0);
    println!("Distance a->b = {}", a.distance(&b));
}
```

---

## src/examples/ownership\_and\_borrowing.rs

```rust
// Demonstrates ownership and borrowing with structs
#[derive(Debug)]
struct Owner {
    name: String,
}

impl Owner {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    // Takes ownership of self and consumes it
    fn consume(self) {
        println!("Consuming Owner with name: {}", self.name);
    }

    // Borrows self immutably
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }

    // Borrows self mutably
    fn rename(&mut self, new: &str) {
        self.name = new.to_string();
    }
}

fn main() {
    let mut o = Owner::new("Alice");
    o.greet();
    o.rename("Bob");
    o.greet();

    let o2 = Owner::new("Carol");
    o2.consume();
    // o2 cannot be used hereafter
}
```

---

## src/examples/derive\_debug\_clone.rs

```rust
#[derive(Debug, Clone)]
struct Config {
    retries: u32,
    enabled: bool,
}

fn main() {
    let cfg = Config { retries: 3, enabled: true };
    let cfg2 = cfg.clone();
    println!("cfg = {:?}, cfg2 = {:?}", cfg, cfg2);
}
```

---

## examples/basic\_struct.rs (runnable example path)

```rust
use rust_structs_teaching::Point;

fn main() {
    let p1 = Point::new(2.0, 3.0);
    let p2 = Point::new(5.0, 7.0);
    println!("Distance = {}", p1.distance(&p2));
}
```

---

## tests/struct\_tests.rs

```rust
use rust_structs_teaching::{Point, Rectangle};

#[test]
fn test_point_distance() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    assert_eq!(a.distance(&b), 5.0);
}

#[test]
fn test_rectangle_area() {
    let r = Rectangle::new(4, 5);
    assert_eq!(r.area(), 20);
}
```

---

## CONTRIBUTING.md

```markdown
# Contributing

Contributions are welcome. Suggested workflow:

1. Fork the repository.
2. Create a feature branch.
3. Add examples or tests.
4. Run `cargo test`.
5. Open a pull request and describe the change.
```

---

## Final notes

This repository is designed to be both a teaching aid and a hands-on starter kit. The examples are intentionally small and self-contained to be readable in a classroom or a short study session. In later modules you can expand this repository with topics such as lifetimes, generics on structs, enum interactions, and advanced ownership patterns.
