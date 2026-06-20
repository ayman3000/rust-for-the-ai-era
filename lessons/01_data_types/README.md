# Lesson 01 — Rust Data Types

Official reference: <https://doc.rust-lang.org/book/ch03-02-data-types.html>

## Goal

Understand Rust's two major families of data types:

1. **Scalar types** — one value
2. **Compound types** — multiple values grouped together

## The Big Idea

Rust is a statically typed language. That means the compiler must know the type of every value at compile time. Most of the time Rust can infer the type, but sometimes you must write it explicitly.

## Scalar Types

Scalar types represent a single value:

| Type | Meaning | Example |
|---|---|---|
| Integer | Whole numbers | `i32`, `u64` |
| Floating-point | Decimal numbers | `f64`, `f32` |
| Boolean | True or false | `bool` |
| Character | One Unicode scalar value | `char` |

## Compound Types

Compound types group multiple values:

| Type | Meaning | Example |
|---|---|---|
| Tuple | Fixed-size group, can mix types | `(i32, f64, bool)` |
| Array | Fixed-size list, same type | `[1, 2, 3]` |

## Key Takeaways

- Rust usually infers types, but annotations are sometimes required.
- Integer types can be signed (`i`) or unsigned (`u`).
- `char` is Unicode, not just ASCII.
- Tuples can hold mixed types.
- Arrays must hold one type and have a fixed length.

## Run the Example

```bash
rustc examples/main.rs -o /tmp/rust_data_types && /tmp/rust_data_types
```

## Your Checkpoint

After watching the video and reading this recap, open the interactive quiz:

[Start the Lesson 01 Quiz](quiz/)
