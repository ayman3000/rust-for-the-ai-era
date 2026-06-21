/*
Lesson 02 — Intentional Error
=============================
TEACHES: Some operations like parse() need a target type annotation.
RUN:     rustc parse_needs_type.rs
EXPECTED: This file should fail because Rust cannot infer the parsed type.
*/

fn main() {
    let guess = "42".parse().expect("Not a number");
    println!("guess = {guess}");
}
