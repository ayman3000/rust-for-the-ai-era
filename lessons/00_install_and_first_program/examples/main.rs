// Lesson 00 — Install Rust and First Program
// Part of: Rust for the AI Era
// Author: Ayman Hamed

fn main() {
    println!("Lesson 00: First Rust Basics");

    // 1) Create an immutable variable.
    let x = 10;
    println!("x = {x}");

    // This would NOT work, because x is immutable:
    // x = 5;

    // 2) Use mut when you want the value to change.
    let mut changeable_x = 10;
    println!("before changeable_x = {changeable_x}");
    changeable_x = 5;
    println!("after changeable_x = {changeable_x}");

    // 3) Copy a simple integer value into another variable.
    let x = 10;
    let y = x;
    println!("x = {x}");
    println!("y = {y}");
}
