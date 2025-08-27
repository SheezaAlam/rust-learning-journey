
##  Rust Constants Example

```rust
fn main() {
    // A constant in Rust: value and type cannot be changed once defined
    const PASS_VAL: u32 = 70;

    // If we run this code using `cargo run`, the output will be:
    // 70
    println!("{}", PASS_VAL);

    // ❌ This will cause an error:
    // Constants cannot be redefined within the same scope.
    const PASS_VAL: u32 = 90; // Error: cannot redefine a constant
    println!("{}", PASS_VAL);
}
```

---

###  Explanation

* `const` is used for defining **constants**.
* Constants must always have an **explicit type** (e.g., `u32`).
* Once defined, a constant’s **value cannot be changed**.
* If you try to **redefine the same constant in the same scope**, the compiler will throw an error like:

  ```
  error[E0428]: the name `PASS_VAL` is defined multiple times
  ```

## Example 1: Basic Shadowing

```rust
fn main() {
    let x = 4;
    println!("{}", x); // Output: 4

    let x = x + 1;
    println!("{}", x); // Output: 5
}
```

### Explanation

* In Rust, you can declare a new variable with the same name using `let`.
* The new variable **shadows** the old one.
* Here, `x` was `4` → then `x + 1` created a new variable with value `5`.

---

## Example 2: Shadowing with Inner Scope

```rust
fn main() {
    let x = 4;
    println!("{}", x); // Output: 4

    {
        let x = x + 2;
        println!("{}", x); // Output: 6 (inner scope shadows outer `x`)
    }

    let x = x + 1;
    println!("{}", x); // Output: 5
}
```

### Explanation

* Inside the block `{ ... }`, a **new `x`** is created (`x + 2 = 6`).
* This inner `x` only exists inside the block.
* Once we exit the block, the inner `x` is dropped.
* Outside, when we do `let x = x + 1`, it uses the original outer `x = 4` → `4 + 1 = 5`.

---
