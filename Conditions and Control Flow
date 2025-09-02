# Day 7 – Conditions & Logical Operators in Rust

## 1. Comparison Operators

Rust supports all the common comparison operators:

* `==` → equal
* `!=` → not equal
* `<`  → less than
* `>`  → greater than
* `<=` → less than or equal
* `>=` → greater than or equal

### Example

```rust
fn main() {
    let cond = 2 > 3;
    println!("{}", cond); 
}
```

**Output**

```
false
```

---

## 2. Type Casting in Comparisons

When comparing two **different data types** (e.g., integer vs float), Rust requires explicit **type casting**.

```rust
fn main() {
    let cond = (2 as f32) <= 2.2;
    println!("{}", cond);
}
```

**Output**

```
true
```

---

## 3. Logical Operators

* `&&` → AND (true only if both conditions are true)
* `||` → OR (true if at least one condition is true)
* `!`  → NOT (flips true ↔ false)

### AND Example

```rust
fn main() {
    let cond = (2 as f32) <= 2.2; // true
    let cond2 = true && cond; // true && true = true
    println!("{}", cond2);
}
```

**Output**

```
true
```

### OR Example

```rust
fn main() {
    let cond = (2 as f32) <= 2.2; // true
    let cond2 = true || cond; // true || true = true
    println!("{}", cond2);
}
```

**Output**

```
true
```

### NOT Example

```rust
fn main() {
    let cond = (2 as f32) <= 2.2; // true
    let cond2 = false || !cond; // false || !true = false
    println!("{}", cond2);
}
```

**Output**

```
false
```

---

## 4. If–Else Conditions

In Rust, conditions must always evaluate to a **boolean (`true` or `false`)**.

### Simple If

```rust
fn main() {
    let food = "fruit";

    if food == "fruit" {
        println!("yumm");
    }
}
```

**Output**

```
yumm
```

### If–Else If–Else

```rust
fn main() {
    let food = "fruit";

    if food == "lemon" {
        println!("yumm");
    } else if food == "juice" {
        println!(":3");
    } else {
        println!("sorry better luck next time");
    }
}
```

**Output**

```
sorry better luck next time
```

---

##  Key Takeaways

* Rust enforces **strict typing** → always cast when comparing different types.
* Logical operators (`&&`, `||`, `!`) work just like in other languages but always return `true` or `false`.
* Conditions in Rust must evaluate to a boolean — you can’t use numbers like in C/Python (`if 1` is invalid).
* `if–else if–else` gives branching logic.
