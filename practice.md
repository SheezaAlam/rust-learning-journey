
## . Practice Questions (Intermediate)

### `practice/ownership-borrowing.md`

1. Write a function that takes ownership of a `String`, prints it, and returns it back.
2. Implement a function that borrows a vector immutably, counts elements > 10, returns the count.
3. Demonstrate how a mutable reference temporarily freezes other immutable references.
4. Show why two mutable references to the same variable cause a compiler error, then fix it.
5. Implement a “word counter” that borrows a `&str` without cloning.
6. Create a function that accepts both owned and borrowed data using generics and `AsRef<str>`.
7. Use `Rc<T>` to share ownership of a struct instance across two functions.
8. Show lifetime annotations on a function returning the longer of two `&str`.
9. Create an example where moving a value into a closure invalidates its use afterward.
10. Explain and demonstrate “copy semantics” vs. “move semantics” with primitive vs. non-primitive types.

---

### `practice/structs.md`

1. Define a `User` struct with builder-style methods.
2. Implement a method on a `Rectangle` struct that checks for overlap with another rectangle.
3. Use `derive(Debug)` to print a struct and implement `Display` manually.
4. Demonstrate ownership transfer when a struct is returned from a function.
5. Implement a `Point3D` struct and a method computing Euclidean distance.
6. Write an enum `VehicleType` and embed it as a field inside a `Vehicle` struct.
7. Serialize and deserialize a struct using `serde` and JSON.
8. Implement `PartialEq` and `Clone` for a custom struct.
9. Write a `Library` struct holding a `Vec<Book>`, add methods `add_book` and `remove_book`.
10. Implement a static “constructor” function returning a struct with default values.

---

### `practice/constants-variables-shadowing.md`

1. Show the difference between `const` and `let` using a mathematical constant.
2. Demonstrate mutability toggle with `mut` and try compiling without it.
3. Illustrate shadowing with type change (int → string).
4. Write a snippet using shadowing to sanitize and normalize input.
5. Define module-wide constants for a config file path.
6. Show how constants improve readability in a calculation.
7. Create a scope where variable shadowing avoids accidental mutation.
8. Demonstrate difference in runtime cost: reusing variable vs. new binding.
9. Use constants for indexing in an array and show compile-time checking.
10. Shadow an immutable variable inside an inner block and print both values.

---

### `practice/control-flow.md`

1. Write a program using `if let` to match an `Option<i32>`.
2. Use `match` with enum variants instead of nested `if` chains.
3. Implement a fizz-buzz using `for` and pattern matching.
4. Show an early return from a loop with `break` returning a value.
5. Implement nested loops printing a multiplication table.
6. Demonstrate loop labels and breaking an outer loop.
7. Use `while let` to iterate through a vector popping elements.
8. Chain conditions with logical operators inside an `if` statement.
9. Demonstrate `match` with guards (e.g., `if x > 10`).
10. Convert imperative `if` logic into an expression returning a value.

---

### `practice/arithmetic-type-casting.md`

1. Convert between integer types safely using `as`.
2. Handle potential overflow by using `checked_add`.
3. Demonstrate truncation when casting float → int.
4. Write a function that adds numbers of different numeric types using generics + traits.
5. Show precision loss with f32 vs. f64.
6. Implement temperature conversion (C ↔ F) handling float rounding.
7. Use wrapping arithmetic (`wrapping_add`) intentionally.
8. Benchmark integer vs float operations using `Instant::now()`.
9. Create a generic “mean” function for slices of numeric data.
10. Show compile error when mismatched types are used, then fix with casting.

---

### `practice/functions.md`

1. Create a function returning multiple values via a tuple.
2. Write a recursive factorial function.
3. Demonstrate higher-order function taking a closure argument.
4. Implement a generic function that works for any type implementing `Display`.
5. Create an inline function with `#[inline]` and benchmark.
6. Show default parameter behavior via optional arguments pattern.
7. Write a function returning another function (closure).
8. Implement a function using lifetimes explicitly.
9. Build a small library crate exposing public functions for arithmetic.
10. Write an example comparing by-value vs. by-reference parameter passing.

---

## 3. `practice/README.md` Template

```markdown
# Rust Practice Exercises

This folder contains **intermediate-level** coding questions organized by topic.  
Each file (`*.md`) matches a topic from the main learning journey.
## How to Use
1. Pick a topic (e.g., `ownership-borrowing.md`).
2. Attempt to solve each exercise in a separate file (`solution_X.rs`) or a playground.
3. Check official Rust docs or your own notes when stuck.
4. Commit your solutions to track progress.

---

| Topic                        | File                        |
|------------------------------|-----------------------------|
| Ownership & Borrowing         | ownership-borrowing.md       |
| Structs                       | structs.md                  |
| Constants, Variables, Shadowing | constants-variables-shadowing.md |
| Control Flow                  | control-flow.md             |
| Arithmetic & Type Casting     | arithmetic-type-casting.md  |
| Functions, Expressions, Statements | functions.md             |
```

