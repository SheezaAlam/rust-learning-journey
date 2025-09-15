
# Rust Learning Journey – Structured Notes & Code

This repository documents my personal journey of learning the **Rust programming language**, structured step by step with concise explanations, runnable examples, and practice exercises.

It is designed not only as my study log but also as a **beginner-friendly crash course** for anyone starting out with Rust who prefers a **progressive, example-driven approach**.

---

##  Purpose

* Build a strong foundation in Rust for future **protocol-level development** and **systems programming**.
* Provide **concise explanations** of major concepts alongside runnable code.
* Evolve this into a **self-contained reference** for Rust learners.
* Reinforce learning through **progressively harder practice questions**.

This repository is for fundamentals. Advanced protocol development projects will be maintained separately.

---

##  Repository Structure

```
rust-learning-journey/
│
├─ topics/             # Notes & runnable code examples for each concept
│   ├─ installation-process/
│   ├─ data-types/
│   ├─ arithmetic-type-casting/
│   ├─ constants-variables-shadowing/
│   ├─ functions-expressions-statements/
│   ├─ conditions-control-flow/
│   ├─ ownership-borrowing/
│   ├─ structs/
│   ├─ input/
│   └─ tools/
│
├─ practice/           # Practice exercises per topic
│   ├─ ownership-borrowing.md
│   ├─ structs.md
│   ├─ constants-variables-shadowing.md
│   ├─ control-flow.md
│   ├─ arithmetic-type-casting.md
│   ├─ functions.md
│   └─ …
│
└─ README.md
```

---

##  Goals

* Develop a **clear mental model** of ownership, borrowing, and lifetimes.
* Gain fluency with **variables, shadowing, constants, and data types**.
* Practice **real-world coding patterns**: control flow, error handling, and struct design.
* Build a **reusable Rust reference** for future projects.
* Transition from **basic syntax** → **modular code** → **networking and async systems**.

---

##  Topics Covered

* Installation & Tooling (`cargo`, `rustup`)
* Data Types & Type Safety
* Arithmetic & Type Casting
* Constants, Variables & Shadowing
* Functions, Expressions & Statements
* Console Input & Output
* Conditions & Control Flow
* Ownership, Borrowing & Lifetimes
* Structs & Methods
* CLI Tools & Utility Patterns
* Enums & Pattern Matching
* Error Handling (with `anyhow`)
* Traits & Abstraction (planned)
* Async & Networking (planned)

---

## How to Use

1. **Clone the repository**

```bash
git clone https://github.com/SheezaAlam/rust-learning-journey.git
cd rust-learning-journey
```

2. **Explore Topics**

   * Each folder in `topics/` contains:

     * `code.rs` → runnable example
     * `NOTES.md` → theory & explanations

3. **Practice Exercises**

   * Located under `practice/`
   * Open the corresponding `.md` file for problems
   * Solve in a `.rs` file or directly on **Rust Playground**

4. **Iterate & Commit**

   * Treat this repo as a **working notebook**.
   * Commit solutions, experiments, and insights as you progress.

---

##  Prerequisites

* Rust stable toolchain (`rustup`, `cargo`)
* Basic command-line familiarity
* Text editor or IDE with Rust support

  * Recommended: **VS Code + rust-analyzer**

Install Rust quickly:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


##  Roadmap

| Phase | Focus Area                       | Status         |
| ----- | -------------------------------- | -------------- |
| 1     | Fundamentals & Syntax            | ✅ Completed    |
| 2     | Ownership & Memory Safety        | ✅ Completed    |
| 3     | Structs & Modular Code           | ✅ Completed    |
| 4     | Enums & Pattern Matching         | ✅ Completed    |
| 5     | Error Handling & Traits          | 🚧 In Progress |
| 6     | Async, Networking, Serialization | 🚧 Planned     |

---

##  Resources

* [The Rust Programming Language (Book)](https://doc.rust-lang.org/book/)
* [Rustlings Exercises](https://github.com/rust-lang/rustlings)
* [Rust Playground](https://play.rust-lang.org/)
* [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

---

##  License

This project is open source under the **MIT License**.

---

##  Contributing

This is primarily a **personal study log**, but suggestions, clarifications, and improvements are welcome via pull requests.

---

### Final Note

> *“Mastery is built through deliberate, consistent practice.”*
> This repository is my commitment to transforming curiosity into competence—one Rust program at a time.

