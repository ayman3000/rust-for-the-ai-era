/*
Lesson 03 — Intentional Error
=============================
TEACHES: A semicolon can turn a return expression into a statement
RUN:     rustc missing_return_value.rs

This file is supposed to fail.
Part of: Rust for the AI Era
Author:  Ayman Hamed | AI Architect & Instructor
*/

fn estimate_tokens(text: &str) -> usize {
    text.len() / 4;
}

fn main() {
    let tokens = estimate_tokens("Explain Rust functions simply.");
    println!("Estimated tokens: {tokens}");
}
