/*
Lesson 03 — Functions for AI Utilities
======================================
TEACHES: Function parameters, return values, expressions, statements
SHOWS:   How small Rust functions become reusable AI utility helpers
RUN:     rustc main.rs -o /tmp/rust_functions_ai_utilities && /tmp/rust_functions_ai_utilities

Part of: Rust for the AI Era
Author:  Ayman Hamed | AI Architect & Instructor
*/

fn print_section(title: &str) {
    println!("
{title}");
    println!("{}", "=".repeat(title.len()));
}

fn clean_prompt(prompt: &str) -> String {
    prompt.trim().to_string()
}

fn estimate_tokens(text: &str) -> usize {
    let rough_estimate = text.len() / 4;

    if rough_estimate == 0 {
        1
    } else {
        rough_estimate
    }
}

fn build_instruction(task: &str, input: &str) -> String {
    format!("Task: {task}

Input:
{input}

Answer clearly and simply.")
}

fn main() {
    print_section("1. Clean a messy prompt");
    let raw_prompt = "   Explain Rust ownership using a simple analogy.   ";
    let cleaned_prompt = clean_prompt(raw_prompt);
    println!("Raw prompt:     {raw_prompt:?}");
    println!("Cleaned prompt: {cleaned_prompt:?}");

    print_section("2. Estimate prompt size");
    let token_estimate = estimate_tokens(&cleaned_prompt);
    println!("Estimated tokens: {token_estimate}");

    print_section("3. Build a reusable AI instruction");
    let instruction = build_instruction("Explain", &cleaned_prompt);
    println!("{instruction}");

    print_section("4. The lesson idea");
    println!("Functions let us give names to useful steps.");
    println!("Today: prompt utilities. Later: Ollama clients, summarizers, and agents.");
}
