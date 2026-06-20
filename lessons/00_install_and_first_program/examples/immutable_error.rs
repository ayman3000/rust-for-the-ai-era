// Lesson 00 — Intentional compiler error
// Run: rustc immutable_error.rs
// This file is supposed to fail so students can see Rust's immutability rule.

fn main() {
    let x = 10;
    x = 5;
    println!("{x}");
}
