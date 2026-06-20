// Lesson 01 — Rust Data Types
// Part of: Rust for the AI Era
// Author: Ayman Hamed
//
// Run:
// rustc main.rs -o /tmp/rust_data_types && /tmp/rust_data_types

fn main() {
    // Scalar types: one value
    let age: u8 = 35;
    let temperature: f64 = 23.5;
    let is_learning_rust: bool = true;
    let omega: char = 'Ω';

    println!("age = {age}");
    println!("temperature = {temperature}");
    println!("is_learning_rust = {is_learning_rust}");
    println!("omega = {omega}");

    // Tuple: fixed-size group, mixed types allowed
    let learner: (&str, u8, bool) = ("Ayman", 35, true);
    let (name, years_old, active) = learner;
    println!("learner = {name}, {years_old}, active: {active}");

    // Array: fixed-size group, same type only
    let scores: [i32; 3] = [90, 85, 92];
    println!("first score = {}", scores[0]);

    // Repeated array syntax
    let zeros = [0; 5];
    println!("zeros = {:?}", zeros);
}
