# Lesson 02 — Variable Types: From Simple to Advanced

Official references:

- Variables and mutability: <https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html>
- Data types: <https://doc.rust-lang.org/book/ch03-02-data-types.html>

## Goal

Learn how Rust connects **variables** with **types** gradually:

1. Rust can infer simple types.
2. You can write type annotations when clarity is needed.
3. `mut` lets a value change, but not its type.
4. Shadowing creates a new variable and can change the type.
5. `const` requires an explicit type and never changes.

## The Big Idea

A Rust variable is a **name bound to a value**. Every value has a type, and Rust must know that type at compile time.

Most beginner code starts like this:

```rust
let age = 30;
let price = 19.99;
let active = true;
let letter = 'R';
```

Rust infers the types:

```text
age    -> i32
price  -> f64
active -> bool
letter -> char
```

## 1. Simple Inference

When the value is obvious, Rust usually understands the type without help.

```rust
let users = 120;      // i32 by default
let score = 98.5;     // f64 by default
let finished = false; // bool
```

This keeps code clean. You do not need to annotate every variable.

## 2. Type Annotations

A type annotation makes the type explicit:

```rust
let users: u32 = 120;
let score: f32 = 98.5;
let letter: char = 'R';
```

Use annotations when:

- Rust cannot infer the type.
- You want a smaller/larger number type.
- You want the code to communicate intent.

Important example:

```rust
let guess: u32 = "42".parse().expect("Not a number");
```

Without `: u32`, `parse()` is ambiguous because Rust does not know which numeric type you want.

## 3. Mutability Does Not Mean Type Changing

`mut` means the **value** can change:

```rust
let mut count: i32 = 10;
count = 11;
count = 12;
```

But the type stays `i32`. This does **not** compile:

```rust
let mut count = 10;
count = "ten";
```

`mut` is not dynamic typing. Rust is still statically typed.

## 4. Shadowing Can Change the Type

Shadowing means creating a **new variable with the same name**:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

The first `spaces` is a string slice: `&str`.
The second `spaces` is a number: `usize`.

This works because the second `let` creates a new binding.

## 5. Constants Are Always Typed

Constants use `const`, must include a type, and cannot use `mut`:

```rust
const MAX_USERS: u32 = 1_000;
```

Good constants are named in `SCREAMING_SNAKE_CASE` and represent values that should not change during the program.

## 6. Beginner to Advanced Mental Model

| Level | Pattern | Example | Meaning |
|---|---|---|---|
| 1 | Inference | `let x = 10;` | Rust picks a type |
| 2 | Annotation | `let x: i64 = 10;` | You choose the type |
| 3 | Mutable | `let mut x = 10; x = 20;` | Same type, new value |
| 4 | Shadowing | `let x = "10"; let x = 10;` | New binding, new type possible |
| 5 | Constant | `const LIMIT: u32 = 100;` | Typed value that never changes |

## Common Mistakes

### Mistake 1: Thinking `mut` allows a type change

```rust
let mut x = 5;
x = "five"; // wrong
```

Rust rejects this because `x` started as an integer.

### Mistake 2: Forgetting annotations with `parse()`

```rust
let guess = "42".parse().expect("Not a number"); // ambiguous
```

Fix:

```rust
let guess: u32 = "42".parse().expect("Not a number");
```

### Mistake 3: Confusing constants and immutable variables

```rust
let x = 10;          // immutable variable, inferred type
const X: i32 = 10;   // constant, explicit type required
```

## Run the Example

```bash
rustc examples/main.rs -o /tmp/rust_variable_types && /tmp/rust_variable_types
```

## Try the Intentional Errors

```bash
rustc examples/mut_cannot_change_type.rs
rustc examples/parse_needs_type.rs
```

These files are supposed to fail. They teach you how Rust protects your program.

## Your Checkpoint

Open the interactive quiz:

[Start the Lesson 02 Quiz](quiz/)
