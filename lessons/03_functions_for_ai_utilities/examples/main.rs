/*
Lesson 03 — Functions for AI Utilities
======================================
TEACHES: Function parameters, return values, expressions, statements
SHOWS:   How small Rust functions become reusable AI utility helpers
RUN:     rustc main.rs -o /tmp/rust_functions_ai_utilities && /tmp/rust_functions_ai_utilities

Part of: Rust for the AI Era
Author:  Ayman Hamed | AI Architect & Instructor
*/

fn print_line(width: usize) {
    println!("{}", "=".repeat(width));
}

fn estimate_tokens(character_count: usize) -> usize {
    let rough_estimate = character_count / 4;

    if rough_estimate == 0 {
        1
    } else {
        rough_estimate
    }
}

fn prompt_fits_context(estimated_tokens: usize, limit: usize) -> bool {
    estimated_tokens <= limit
}

fn build_default_instruction() -> String {
    String::from("Answer clearly, simply, and with one practical example.")
}

fn main() {
    println!("1. Functions can organize AI utility steps");
    print_line(46);

    let prompt_character_count: usize = 184;
    let context_limit: usize = 120;

    let estimated_tokens = estimate_tokens(prompt_character_count);
    let fits_limit = prompt_fits_context(estimated_tokens, context_limit);
    let instruction = build_default_instruction();

    println!("Prompt character count: {prompt_character_count}");
    println!("Estimated tokens: {estimated_tokens}");
    println!("Context limit: {context_limit}");
    println!("Fits limit? {fits_limit}");

    println!("\n2. Default AI instruction");
    print_line(25);
    println!("{instruction}");

    println!("\n3. The lesson idea");
    print_line(18);
    println!("Functions let us give names to useful steps.");
    println!("Today: simple inputs and outputs. Later: borrowing lets us pass real prompt text.");
}
