# Lesson 03 — Functions for AI Utilities

Official references:

- Functions: <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html>
- Comments: <https://doc.rust-lang.org/book/ch03-04-comments.html>

## Goal

Learn Rust functions without mixing in ownership or borrowing yet.

By the end, you will understand:

1. How to define a function with `fn`.
2. How parameters receive simple values.
3. How return types work with `->`.
4. The difference between statements and expressions.
5. How tiny functions become useful building blocks for later AI tools.

## Important Teaching Boundary

This lesson intentionally avoids string references and borrowing.

Why?

Because learners should first understand the shape of a function:

```text
input values → named function → output value
```

Borrowing comes later, when students ask the natural question:

> How do we pass real prompt text into functions without copying or moving it?

That question belongs to the borrowing lesson.

## The Big Idea

A function is a small named action.

In this course, functions are not just syntax. They are how we organize useful steps while building AI tools.

For now, we keep the inputs simple:

- numbers like `usize`
- numbers like `u32`
- booleans like `bool`
- returned owned text like `String`

This keeps the lesson focused.

## 1. A Function with No Return Value

```rust
fn print_line(width: usize) {
    println!("{}", "=".repeat(width));
}
```

This function does one job: print a separator line.

It has:

- a name: `print_line`
- one parameter: `width`
- one parameter type: `usize`
- no return value

No borrowing needed.

## 2. Parameters Need Types

Rust does not let function parameters be vague.

This is correct:

```rust
fn estimate_tokens(character_count: usize) -> usize {
    character_count / 4
}
```

This is not allowed:

```rust
fn estimate_tokens(character_count) -> usize {
    character_count / 4
}
```

Rust wants to know what type `character_count` is.

That strictness is useful. When we build AI tools later, clear input types prevent confusion.

## 3. Returning a Value

A return type is written after an arrow:

```rust
fn estimate_tokens(character_count: usize) -> usize {
    character_count / 4
}
```

The last line has no semicolon.

That means it is an expression, and Rust returns it.

```rust
character_count / 4
```

## 4. Statement vs Expression

This returns a number:

```rust
fn good() -> usize {
    10
}
```

This does **not** return a number:

```rust
fn bad() -> usize {
    10;
}
```

The semicolon turns `10` into a statement. A statement does work but does not become the returned value.

This is one of the most important beginner Rust ideas.

## 5. Practical AI Utility Functions

In the runnable example, we build four useful helpers:

| Function | Job |
|---|---|
| `print_line(width)` | Make terminal output readable |
| `estimate_tokens(character_count)` | Estimate model context size simply |
| `prompt_fits_context(estimated_tokens, limit)` | Decide if a prompt is short enough |
| `build_default_instruction()` | Return a reusable prompt instruction |

The examples are intentionally simple. Later, borrowing lets these functions work with real prompt text.

## Run the Example

```bash
rustc examples/main.rs -o /tmp/rust_functions_ai_utilities && /tmp/rust_functions_ai_utilities
```

Expected idea:

```text
Prompt character count: 184
Estimated tokens: 46
Fits limit? true
```

## Try the Intentional Error

```bash
rustc examples/missing_return_value.rs
```

This file is supposed to fail. It shows why a semicolon can accidentally remove a function's return value.

## Your Checkpoint

Open the interactive quiz:

[Start the Lesson 03 Quiz](quiz/)

## Small Challenge

Add a new function:

```rust
fn double_limit(limit: usize) -> usize
```

It should return:

```rust
limit * 2
```

Keep it simple. Text input comes later when borrowing is introduced.
