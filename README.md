
# Rust Learning Journey – Crash Course with Code & Notes

This repository is a curated record of my personal journey learning the Rust programming language.  
It grows chapter by chapter as I explore core Rust concepts, document key theory, and create runnable code examples.  
The goal is to make it useful not only for me, but for anyone starting out with Rust who prefers a **progressive, example-driven** approach.

---

## 1. Purpose

- Build a **solid foundation** in Rust for future protocol-level projects and systems programming.
- Provide **concise explanations** of major concepts alongside working code.
- Evolve this into a **beginner-friendly crash course** with practice questions and exercises.

> *This repo focuses on learning and teaching fundamentals. Advanced, production-grade protocol projects will live in a separate repository later.*
A structured, code-driven walkthrough of the Rust programming language.  
This repository documents a progressive study path—starting from installation and the Rust toolchain, moving through language fundamentals, and culminating in intermediate topics and hands-on exercises.

---

##  Repository Structure

```

rust-learning-journey/
│
├─ topics/            # Notes & code examples for each concept
│   ├─ install-process/
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
├─ practice/          # Intermediate exercises per topic
│   ├─ ownership-borrowing.md
│   ├─ structs.md
│   ├─ constants-variables-shadowing.md
│   ├─ control-flow\.md
│   ├─ arithmetic-type-casting.md
│   ├─ functions.md
│   └─ …
│
└─ README.md          

````

---

##  Goals

- Build a clear mental model of Rust’s ownership, borrowing, and lifetime rules.
- Develop fluency with variables, shadowing, constants, and core data types.
- Practice real-world patterns: control flow, error handling, and struct design.
- Reinforce concepts through targeted, progressively harder practice questions.
- Establish a reusable reference for future Rust work or onboarding.

---

##  Topics Covered

- Installation & Tooling (Cargo, rustup)
- Data Types & Type Safety
- Arithmetic & Type Casting
- Constants, Variables & Shadowing
- Functions, Expressions & Statements
- Console Input & Output
- Conditions & Control Flow
- Ownership, Borrowing & Lifetimes
- Structs & Methods
- CLI Tools & Utility Patterns
- …and more coming soon (Enums, Error Handling, Traits, Modules, Async, etc.)

---

##  How to Use

1. **Clone the repository**
   ```bash
   git clone https://github.com/SheezaAlam/rust-learning-journey.git
   cd rust-learning-journey


2. **Explore Topics**
   Each folder in `topics/` contains:

   * `code.rs` — concise example code
   * `NOTES.md` — quick theory or commentary

3. **Practice**
   Exercises live under `practice/`.

   * Open the matching `.md` file for a list of problems
   * Solve each in a standalone `.rs` file or on [Rust Playground](https://play.rust-lang.org)

4. **Iterate & Commit**
   Treat this repo as a working notebook. Commit solutions, experiments, and insights as you go.

---

##  Prerequisites

* Rust stable toolchain (`rustup`, `cargo`)
* Basic command-line familiarity
* A text editor or IDE with Rust support (VS Code + rust-analyzer recommended)

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

##  Roadmap

| Phase | Focus                            | Status     |
| ----- | -------------------------------- | ---------- |
| 1     | Fundamentals & Syntax            | ✅          |
| 2     | Ownership & Memory Safety        | ✅          |
| 3     | Structs & Modular Code           | ✅          |
| 4     | Enums, Pattern Matching          | ✅ |
| 5     | Error Handling & Traits          | 🚧 Planned |
| 6     | Async, Networking, Serialization | 🚧 Planned |

---

##  License

This project is open source under the [MIT License](LICENSE) (add a license file if you want contributions).

---

##  Contributing

This repository is primarily a personal study log, but improvements, fixes, and clarifications are welcome via pull request.

---

##  Resources

* [Rust Book (Official)](https://doc.rust-lang.org/book/)
* [Rustlings Exercises](https://github.com/rust-lang/rustlings)
* [Rust Playground](https://play.rust-lang.org)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)


> *“Mastery is the result of consistent, deliberate practice.”*
> This repository aims to turn curiosity into competence through step-by-step coding.

