/*
Lesson 02 — Variable Types
==========================
TEACHES: Type inference, type annotations, mutability, shadowing, constants
SHOWS:   How Rust connects every variable to a compile-time type
RUN:     rustc main.rs -o /tmp/rust_variable_types && /tmp/rust_variable_types

Part of: Rust for the AI Era
Author:  Ayman Hamed | AI Architect & Instructor
*/

const MAX_USERS: u32 = 1_000;

fn section(title: &str) {
    println!("\n{title}");
    println!("{}", "=".repeat(title.len()));
}

fn main() {
    section("1. Rust can infer simple types");
    let age = 30;          // default integer: i32
    let price = 19.99;     // default float: f64
    let active = true;     // bool
    let letter = 'R';      // char
    println!("age = {age}");
    println!("price = {price}");
    println!("active = {active}");
    println!("letter = {letter}");

    section("2. You can write the type explicitly");
    let users: u32 = 120;
    let temperature: i32 = -5;
    let ratio: f32 = 0.75;
    println!("users: u32 = {users}");
    println!("temperature: i32 = {temperature}");
    println!("ratio: f32 = {ratio}");

    section("3. parse() usually needs a target type");
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess parsed as u32 = {guess}");

    section("4. mut changes the value, not the type");
    let mut count: i32 = 10;
    println!("before count = {count}");
    count = 11;
    println!("after count = {count}");

    section("5. shadowing can create a new type");
    let spaces = "   ";
    println!("spaces as text has length = {}", spaces.len());
    let spaces = spaces.len();
    println!("spaces is now a usize value = {spaces}");

    section("6. constants require explicit types");
    println!("MAX_USERS: u32 = {MAX_USERS}");
}
