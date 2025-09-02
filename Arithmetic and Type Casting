# Rust Learning 

### Topic: Console Input, Comments, Type Casting, and Integer Conversion

## 1. Comments in Rust

* `//` is used for single-line comments.
* Comments are ignored by the compiler but help explain the code.

```rust
// This is a single-line comment
fn main() {
    println!("Hello, world!"); // This comment is at the end of a line
}
```

---

## 2. Integer Overflow

* `u8` ranges from **0 to 255**.
* If you exceed this range, it causes an **overflow error** in debug mode.

```rust
fn main() {
    let x: u8 = 255;
    let y = x + 1; // Will cause overflow in debug mode
    println!("{}", y);
}
```

---

## 3. Type Casting

* Rust is strict about type safety.
* You cannot add different integer types directly (e.g., `u8 + i8`).
* Solution: convert smaller type into the larger type.

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 20;

    let sum = a as i64 + b; // Casting a to i64
    println!("Sum = {}", sum);
}
```

---

## 4. Taking User Input and Converting to Integer

* Input in Rust is taken as a **String** by default.
* Use `trim()` to remove spaces/newline, `parse()` to convert, and `unwrap()` to handle errors.

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i64 = input.trim().parse().unwrap();
    println!("You entered: {}", number);
}
```

---

## 5. Key Learnings

* `//` → comments.
* Rust enforces type safety.
* Always cast integers before mixing types.
* Input must be parsed from `String` to desired type.
* Overflow errors occur if values exceed type limits.

---

