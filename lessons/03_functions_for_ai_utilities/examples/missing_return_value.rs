/*
Lesson 03 — Intentional Error
=============================
TEACHES: A semicolon can turn a return expression into a statement
RUN:     rustc missing_return_value.rs

This file is supposed to fail.
Part of: Rust for the AI Era
Author:  Ayman Hamed | AI Architect & Instructor
*/

fn estimate_tokens(character_count: usize) -> usize {
    character_count / 4;
}

fn main() {
    let tokens = estimate_tokens(184);
    println!("Estimated tokens: {tokens}");
}
