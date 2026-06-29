# Lesson 00 — Install Rust and Write Your First Program

**Slogan:** Rust starts strict so your programs become strong.

This is the first lesson before data types. We keep it short and practical: install Rust, create `main`, store a value in `x`, print it, then understand why Rust does not let you change `x` unless you ask for mutability.

## 1. Install Rust

Use the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or run:

```bash
source "$HOME/.cargo/env"
```

Check the installation:

```bash
rustc --version
cargo --version
```

## 2. Your first Rust file

Create a file named `main.rs`:

```rust
fn main() {
    let x = 10;
    println!("{x}");
}
```

Run it:

```bash
rustc main.rs
./main
```

Expected output:

```text
10
```

## 3. What is `fn main()`?

```rust
fn main() {
    // your code starts here
}
```

`main` is the starting point of a Rust program. When you run the program, Rust enters `main` first.

## 4. `let x = 10;`

```rust
let x = 10;
```

This creates a variable named `x` and stores the value `10` inside it.

By default, Rust variables are **immutable**. That means you cannot change them after creation.

This will not compile:

```rust
fn main() {
    let x = 10;
    x = 5;
    println!("{x}");
}
```

Rust rejects it because `x` was not declared as mutable.

## 5. Use `mut` when you want to change a variable

```rust
fn main() {
    let mut x = 10;
    println!("before: {x}");

    x = 5;
    println!("after: {x}");
}
```

Expected output:

```text
before: 10
after: 5
```

`mut` is a clear signal: **this variable is allowed to change**.

## 6. Copy a value: `let y = x;`

After you understand mutability, try assigning one variable to another:

```rust
fn main() {
    let x = 10;
    let y = x;

    println!("x = {x}");
    println!("y = {y}");
}
```

Expected output:

```text
x = 10
y = 10
```

For simple numbers like integers, Rust copies the value. So both `x` and `y` can be printed.

## Important syntax note

Use this:

```rust
println!("{x}");
```

Not this:

```rust
println!({x});
```

`println!` needs a format string, and `"{x}"` is the format string that says: print the value of `x`.

## Recap

- Install Rust with `rustup`.
- `fn main()` is where the program starts.
- `let x = 10;` creates an immutable variable.
- `x = 5;` fails if `x` is not mutable.
- `let mut x = 10;` allows `x` to change.
- `let y = x;` copies simple values like integers.
- `println!("{x}");` prints the value of `x`.

## Run the lesson examples

```bash
cd lessons/00_install_and_first_program/examples
rustc main.rs -o lesson00
./lesson00
```

## Try the intentional errors

These files are supposed to fail. They teach you how Rust protects your program:

```bash
rustc immutable_error.rs            # x = 5 on an immutable variable
rustc immutable_increment_error.rs   # x = x + 1 on an immutable variable
```

The compiler message is part of the lesson.
