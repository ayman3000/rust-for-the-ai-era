# Rust for the AI Era — Course Roadmap

## Strategy

This course is for beginners who want Rust to become a practical tool for modern software and AI-era systems.

## Core Path

0. Install Rust and First Program
1. Data Types
2. Variable Types
3. Functions
4. Control Flow
5. Ownership
6. Borrowing and References
7. Structs
8. Enums and Pattern Matching
9. Error Handling
10. Collections
11. Traits and Generics
12. Modules and Packages
13. CLI Tools
14. APIs and Services
15. Rust for AI Tooling

## Lesson 00 Scope

Lesson 00 is intentionally short. It introduces:

- installing Rust with `rustup`
- checking `rustc` and `cargo`
- `fn main()` as the program entry point
- `let x = 10;`
- correct printing with `println!("{x}");`
- the immutable-by-default rule
- changing values with `let mut x`
- copying a simple value with `let y = x`

## Lesson 02 Scope

Lesson 02 teaches variable types gradually, from simple to advanced:

- type inference with `let age = 30`
- explicit annotations like `let users: u32 = 120`
- `parse()` needing a target type such as `u32`
- `mut` changing values but not types
- shadowing as a new binding that can change type
- constants requiring explicit types and `SCREAMING_SNAKE_CASE`

## Per-Lesson Deliverables

Each lesson should include:

- `README.md` lesson recap
- `examples/` runnable Rust snippets
- `quiz/` interactive HTML/CSS/JS quiz
- optional PDF recap
- source reference links

## Quiz Rules

- Questions must be course-relevant and non-trivial.
- Distractors must be plausible Rust misunderstandings.
- Correct answers must be spread across A/B/C/D.
- Each question should teach something even when the learner answers wrong.
