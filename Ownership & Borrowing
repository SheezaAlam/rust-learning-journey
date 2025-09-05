
# Rust Ownership & Borrowing – Teaching Notes

This repository demonstrates how Rust manages memory safety without a garbage collector through its **ownership model**. The examples cover:

- Ownership transfer (move semantics)
- Borrowing with immutable references (`&T`)
- Why borrowing is useful for efficiency and safety

---

## 1. Overview

Rust enforces three core rules at compile time:

1. Every value has a single **owner**.
2. When the owner goes **out of scope**, the value is dropped (memory freed).
3. Ownership can be **moved** or **borrowed**; at any given time you can have:
   - One **mutable** reference _or_
   - Any number of **immutable** references, but not both.

These rules prevent **use-after-free**, **double free**, and **data race** errors at compile time.

---

## 2. Code Examples

### Ownership Transfer

```rust
fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);      // move into the function

    // println!("{}", s1);   // error: value moved
}

fn take_ownership(s: String) {
    println!("Got: {}", s);
} // s is dropped here
````

### Borrowing (Immutable Reference)

```rust
fn main() {
    let s1 = String::from("hello");
    print_length(&s1);       // borrow, no move
    println!("{}", s1);      // s1 is still valid
}

fn print_length(s: &String) {
    println!("Length = {}", s.len());
}
```

---

## 3. Why Borrow Instead of Move?

* Avoids **unnecessary copying** of large data
* Allows **multiple parts of code to read** a value safely
* Preserves ownership with the caller while letting other functions **inspect** data

Borrowing is ideal when a function only needs to **read** or temporarily **access** a value without taking responsibility for its lifetime.

---

## 4. Suggested Exercises

1. Modify `print_length` to accept a **mutable** reference (`&mut String`) and append text.
2. Write a function that **returns** ownership back to the caller after processing.
3. Experiment with creating **two mutable references** in the same scope and observe the compiler error.

---

## 5. Learning Resources

* [The Rust Programming Language (Book) – Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
* [Rust Reference – Borrowing](https://doc.rust-lang.org/reference/expressions.html#borrowing)
* [Rust Playground](https://play.rust-lang.org/) – experiment with code online

---

## 6. License

This material is provided for educational purposes under the MIT License. Contributions and suggestions are welcome.

```

