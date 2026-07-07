# Lesson 03 — Functions for AI Utilities

Official references:

- Functions: <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html>
- Comments: <https://doc.rust-lang.org/book/ch03-04-comments.html>

## Goal

Learn Rust functions by building tiny utilities that will become useful later when we talk to Ollama.

By the end, you will understand:

1. How to define a function with `fn`.
2. How parameters receive values.
3. How return types work with `->`.
4. The difference between statements and expressions.
5. Why small functions make AI tools easier to build and test.

## The Big Idea

A function is a small named action.

In this course, functions are not just syntax. They are how we stop repeating ourselves when building AI tools.

Instead of writing messy code like this everywhere:

```rust
let prompt = "   Explain Rust ownership in one paragraph.   ";
let prompt = prompt.trim();
println!("Prompt: {prompt}");
println!("Estimated tokens: {}", prompt.len() / 4);
```

We create reusable helpers:

```rust
let cleaned = clean_prompt(prompt);
let tokens = estimate_tokens(&cleaned);
```

This is simple, but powerful.

## 1. Your First Utility Function

```rust
fn print_section(title: &str) {
    println!("
{title}");
    println!("{}", "=".repeat(title.len()));
}
```

This function does one job: print a clean section title.

It has:

- a name: `print_section`
- one parameter: `title`
- one parameter type: `&str`
- no return value

We will use this helper in many future examples because readable output helps students debug.

## 2. Parameters Need Types

Rust does not let function parameters be vague.

This is correct:

```rust
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}
```

This is not allowed:

```rust
fn estimate_tokens(text) -> usize {
    text.len() / 4
}
```

Rust wants to know what type `text` is.

That strictness is useful. When we build AI tools, clear input types prevent confusion.

## 3. Returning a Value

A return type is written after an arrow:

```rust
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}
```

The last line has no semicolon.

That means it is an expression, and Rust returns it.

```rust
text.len() / 4
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
| `print_section(title)` | Make terminal output readable |
| `clean_prompt(prompt)` | Trim messy user input |
| `estimate_tokens(text)` | Estimate model cost/context size simply |
| `build_instruction(task, input)` | Create a reusable prompt template |

The examples are intentionally simple. Later, these ideas grow into Ollama requests, file summarizers, and agents.

## Run the Example

```bash
rustc examples/main.rs -o /tmp/rust_functions_ai_utilities && /tmp/rust_functions_ai_utilities
```

Expected idea:

```text
1. Clean a messy prompt
=======================
Raw prompt:     "   Explain Rust ownership using a simple analogy.   "
Cleaned prompt: "Explain Rust ownership using a simple analogy."
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
fn make_short_answer_prompt(question: &str) -> String
```

It should return a prompt like:

```text
Answer this in three short bullet points: <question>
```

Keep it simple. If the function runs, you can improve it.
