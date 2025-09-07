## Data Types in Rust

In Rust, **primitive data types** are the basic building blocks. They are divided into two categories:

### 1. Scalar Types

Represent a **single value**.

* **Integers** → `i32` (default, signed: positive + negative), `u32` (unsigned: only positive).
* **Floating Point Numbers** → `f32` (32-bit), `f64` (64-bit, default).
* **Booleans** → `true` or `false`.
* **Characters** → Represent a single Unicode scalar value (e.g., `'a'`, `'A'`, `'1'`, `'$'`).

### 2. Compound Types

Group multiple values into one type.

* **Arrays** → Fixed-size collection of values of the same type.
* **Tuples** → Group values of different types, fixed size, immutable by default.

---

### Example Code

```rust
fn main() {
    // u32 = only positive integers
    let x: u32 = 34;
    println!("{}", x); // Output: 34

    // f32 = floating point number (f64 also available)
    let y: f32 = 4.5;
    println!("{}", y); // Output: 4.5

    // Boolean: true or false
    let z: bool = true;
    println!("{}", z); // Output: true

    // Character: any single Unicode char
    let a: char = 'a';
    println!("{}", a); // Output: a

    // Array: fixed-size collection
    let arr: [i32; 4] = [1, 3, 4, 5];
    println!("{}", arr[2]); // Output: 4

    // Tuple: group multiple types
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.1); // Output: true
}
```

---

### Output

```
34
4.5
true
a
4
true
```

