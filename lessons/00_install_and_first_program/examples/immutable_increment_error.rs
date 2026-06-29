// Lesson 00 — Intentional compiler error
// Run: rustc immutable_increment_error.rs
// This file is supposed to fail so students can see Rust's immutability rule.

fn main() {
    let x = 6;
    x = x + 1; // ERROR: x is immutable
    println!("{x}");
}