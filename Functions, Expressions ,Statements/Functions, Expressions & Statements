# Rust Learning 

### Topic: Functions, Expressions, Statements, and Return Values

---

## 1. Functions in Rust

* A **function** is a block of code that performs a task.
* Rust requires functions to have explicitly declared parameter types and return types.
* The **main function** (`fn main()`) is the entry point of every Rust program.

Example:

```rust
fn main() {
    // This is the entry point
    println!("hi");

    // Calling the function
    add_numbers(2, 4);

    // Storing the returned value from function into a variable
    let sum = add_numbers(2, 3);
    println!("The returned sum is: {}", sum);
}

// Define a function
fn add_numbers(x: i32, y: i32) -> i32 {
    // Parameters with explicit data types
    let result = x + y;

    // Printing inside the function
    println!("Inside function: {}", result);

    // Returning the value
    result
}
```

### Output:

```
hi
Inside function: 6
Inside function: 5
The returned sum is: 5
```

---

## 2. Expressions vs. Statements

### Statements:

* Perform an action but **do not return a value**.
* Example:

  ```rust
  let x = 5; // Statement
  ```

### Expressions:

* Evaluate to a value and **can be used inside statements**.
* Example:

  ```rust
  let y = 2 + 3;  // Here, `2 + 3` is an expression returning 5
  ```

### Inside functions:

* The **last line without a semicolon** is treated as an **expression** → it returns a value.
* If you add a semicolon (`;`), it becomes a statement and does not return.

```rust
fn square(num: i32) -> i32 {
    num * num   // expression, returns num squared
}
```

If you mistakenly write:

```rust
fn square(num: i32) -> i32 {
    num * num;  // statement, does NOT return → will cause an error
}
```

---

## 3. Why Functions?

* **Avoid repetition** – instead of writing the same code multiple times, define a function and call it.
* **Organize code** – makes programs easier to read and maintain.
* **Reusability** – the same function can be used for different inputs.

---

## 4. Key Learnings

* `fn` keyword defines a function.
* Every parameter and return type must be explicitly typed.
* The last expression in a function is automatically returned.
* **Statements** = do something, no return value.
* **Expressions** = evaluate to a value, can be returned.

--
