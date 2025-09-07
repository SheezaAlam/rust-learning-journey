---

###  Proposed Structure

```
Error_Handling/
│── README.md
│── basic_match.rs
│── question_mark.rs
│── unwrap_expect.rs
│── if_let_ok.rs
│── custom_error.rs
```


# Error Handling in Rust

Error handling ensures that a program can respond to unexpected situations without crashing.
Rust makes errors explicit using the `Result<T, E>` type instead of exceptions.

This module covers the **practical patterns** you will see most often in real codebases.

---

## Key Ideas

- `Result<T, E>` means “either **Ok(T)** on success or **Err(E)** on failure.”
- Handling errors forces you to decide: **recover**, **propagate**, or **abort**.
- Idiomatic Rust prefers returning `Result` instead of silently failing.

---

## 1. Handling with `match`

```rust
// basic_match.rs
use std::fs;

fn main() {
    match fs::read_to_string("hello.txt") {
        Ok(text) => println!("File says:\n{}", text),
        Err(e) => eprintln!("Could not read file: {}", e),
    }
}
````

Explicit, clear, beginner-friendly.

---

## 2. Propagating with `?`

```rust
// question_mark.rs
use std::fs;
use std::io;

fn read_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("hello.txt")?; // auto-return on Err
    Ok(content)
}

fn main() {
    match read_file() {
        Ok(data) => println!("Data: {}", data),
        Err(err) => eprintln!("Problem: {}", err),
    }
}
```

`?` is the shorthand for “if error, return it immediately.”

---

## 3. `unwrap` and `expect` (Panic on Error)

```rust
// unwrap_expect.rs
use std::fs;

fn main() {
    // Panics if hello.txt is missing
    let content = fs::read_to_string("hello.txt").unwrap();

    // Same, but with a custom panic message
    let content = fs::read_to_string("hello.txt")
        .expect("Failed to open hello.txt");

    println!("Content: {}", content);
}
```

Only use for quick experiments or guaranteed-success code.

---

## 4. Success-Only Branch with `if let Ok(..)`

```rust
// if_let_ok.rs
use std::fs;

fn main() {
    if let Ok(text) = fs::read_to_string("hello.txt") {
        println!("We got text: {}", text);
    } // silently ignores errors
}
```

Handy when failure is acceptable and you only care about success.

---

## 5. Flexible Return Type

```rust
// custom_error.rs
use std::fs;
use std::error::Error;

fn read_file() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string("hello.txt")?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_file()?;
    println!("Data: {}", data);
    Ok(())
}
```

Returning `Box<dyn Error>` allows “any error” to propagate—common in libraries and CLI apps.

---

## Quick Reference Table

| Style            | Purpose                               |
| ---------------- | ------------------------------------- |
| `match`          | Explicit, great for branching logic   |
| `?`              | Idiomatic way to bubble up errors     |
| `unwrap`         | Crash; quick prototyping only         |
| `expect`         | Crash + helpful message               |
| `if let`         | Success-only branch, ignore errors    |
| `Box<dyn Error>` | Flexible return type for mixed errors |

---

---

**License**
MIT License – feel free to copy and adapt.

```
