/*
Lesson 02 — Intentional Error
=============================
TEACHES: `mut` lets a value change, but the variable keeps the same type.
RUN:     rustc mut_cannot_change_type.rs
EXPECTED: This file should fail with a type mismatch error.
*/

fn main() {
    let mut count = 10;
    count = "ten";
    println!("{count}");
}
