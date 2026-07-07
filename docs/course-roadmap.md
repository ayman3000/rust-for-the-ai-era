# Rust for the AI Era — Course Roadmap

## Course Strategy

This is **not** a traditional Rust course.

Traditional Rust courses usually teach the language in isolation:

```text
variables → functions → control flow → ownership → structs → enums → traits → modules → maybe a project
```

This course teaches Rust the way modern builders need it:

```text
small Rust foundation → ownership/borrowing → Ollama projects → new Rust concepts exactly when the project needs them
```

The promise is simple:

> Learn Rust by building useful local AI tools with Ollama — gradually, practically, and without unnecessary theory.

## Teaching Philosophy

### Simple but Impactful

Every lesson must be simple enough for a beginner to follow, but useful enough that the student feels they built something real.

A lesson is only successful if the student can say:

> I understand one new Rust concept, and I used it to build something practical.

### Explain While Building

The order is always:

1. Build a tiny working example.
2. Run it immediately.
3. Explain the Rust idea inside the working example.
4. Show one thing that can fail.
5. Improve the code slightly.

Avoid long theory-first lessons. Rust concepts should appear when the student needs them.

### Gradual Examples

Each lesson should move in small steps:

```text
tiny example → slightly useful example → practical AI-era example
```

For example, do not teach `struct` as an abstract language feature. Teach it when students need to model an Ollama JSON request.

### Local First

The AI examples should use local tools first:

- Ollama
- local models
- local files
- local command-line tools

No paid API key should be required for the core path.

## Audience

This course is for beginners and early intermediate developers who want Rust to feel useful quickly.

The course should work for:

- Udemy students who want visible results fast
- Arabic/MENA learners who prefer local/free tools
- AI builders who already understand why Ollama and local LLMs matter
- Python developers who want a systems-language upgrade

## Course Spine

## Phase 1 — Rust Survival Kit

The first phase gives students enough Rust to read code, write small programs, and understand ownership.

### Lesson 00 — Install Rust & First Program

**Status:** built

**Goal:** Run the first Rust program and understand the minimum syntax.

**Teaches:**

- `rustup`
- `rustc`
- `cargo`
- `fn main()`
- `println!`
- `let`
- immutability
- `mut`

**Practical angle:**

Students learn that Rust is strict, but friendly when they read compiler feedback.

### Lesson 01 — Data Types

**Status:** built

**Goal:** Understand the basic shapes of values Rust can store.

**Teaches:**

- integers
- floats
- booleans
- characters
- tuples
- arrays
- type inference

**Practical angle:**

Students learn to recognize the kinds of data they will later send to AI tools.

### Lesson 02 — Variable Types

**Status:** built

**Goal:** Learn when Rust can infer types and when the programmer must be explicit.

**Teaches:**

- inferred types
- explicit annotations
- `parse()` needing a target type
- `mut` changes values but not types
- shadowing
- constants

**Practical angle:**

Students learn why ambiguous input text needs clear target types.

### Lesson 03 — Functions for AI Utilities

**Status:** planned

**Goal:** Turn repeated code into small reusable utilities.

**Teaches:**

- function parameters
- return values
- expressions vs statements
- `&str` parameters as a gentle preview of borrowing

**Gradual examples:**

1. `print_section(title)`
2. `clean_prompt(prompt)`
3. `estimate_tokens(text)`

**Practical AI-era example:**

Build small helper functions used later by Ollama prompt tools.

### Lesson 04 — Control Flow for Prompt Decisions

**Status:** planned

**Goal:** Make simple decisions before sending work to an AI model.

**Teaches:**

- `if` / `else`
- `for`
- `while`
- `loop`
- beginner-friendly `match`

**Gradual examples:**

1. Check whether a prompt is empty.
2. Warn if a prompt is too long.
3. Loop through example prompts and classify them.

**Practical AI-era example:**

Build a tiny prompt validator that decides whether text is ready to send to a model.

### Lesson 05 — Ownership: Who Owns the Data?

**Status:** planned; Manim visuals already started

**Goal:** Understand Rust's most important mental model.

**Teaches:**

- stack vs heap intuition
- `String` vs simple copy types
- move
- clone
- drop
- scope

**Visual analogies:**

- Ownership = one house, one owner
- Move = transfer the deed
- Drop = the house is cleaned up when the owner leaves scope

**Practical AI-era example:**

Show what happens when prompt text is moved into a function, then explain when to clone and when not to.

### Lesson 06 — Borrowing: Passing Prompts Without Copies

**Status:** planned; Manim visuals already started

**Goal:** Pass data around without giving up ownership.

**Teaches:**

- references
- `&T`
- `&mut T`
- many readers or one writer
- avoiding unnecessary copies

**Visual analogies:**

- Borrowing = borrowing a book
- Mutable borrowing = borrowing car keys
- Borrowing rules = many readers or one writer, not both

**Practical AI-era example:**

Pass prompt text into multiple helper functions without moving or cloning the original `String`.

## Phase 2 — The AI Era Starts Here: Rust + Ollama

After borrowing, the course changes style. New Rust concepts are introduced through Ollama projects.

### Lesson 07 — First Ollama Request from Rust

**Goal:** Make Rust talk to a local LLM.

**Teaches Rust:**

- `Cargo.toml`
- external crates
- `reqwest`
- `serde_json`
- async entry point with `tokio` at a simple level

**Teaches AI:**

- Ollama local API
- model name
- prompt
- response
- local inference

**Student builds:**

A CLI program that sends one prompt to Ollama and prints the response.

**Deliverable:**

```bash
cargo run
```

prints a real answer from a local Ollama model.

### Lesson 08 — Structs by Modeling Ollama JSON

**Goal:** Give data a clear shape.

**Teaches Rust:**

- `struct`
- named fields
- `#[derive(Serialize)]`
- `#[derive(Deserialize)]`
- `Debug`

**Teaches AI:**

- request body
- response body
- JSON contracts

**Student builds:**

Typed request and response structs for Ollama.

**Key teaching line:**

> Structs are not just syntax. Structs are how Rust gives shape to real data.

### Lesson 09 — Error Handling When Ollama Fails

**Goal:** Make failure visible and understandable.

**Teaches Rust:**

- `Result`
- `?`
- `match`
- readable errors
- `anyhow` or simple boxed errors

**Teaches AI:**

- Ollama not running
- model missing
- network failure
- invalid response

**Student builds:**

A safer Ollama client that explains what went wrong.

**Intentional failures:**

- wrong model name
- Ollama server stopped
- malformed URL

### Lesson 10 — Enums by Creating AI Task Types

**Goal:** Route different AI tasks clearly.

**Teaches Rust:**

- `enum`
- variants
- `match`
- task routing

**Teaches AI:**

- summarization prompt
- translation prompt
- explanation prompt

**Student builds:**

A small program that chooses between tasks:

```bash
cargo run summarize
cargo run translate
cargo run explain
```

**Key teaching line:**

> Enums let your program choose its mission.

### Lesson 11 — Collections by Building Prompt History

**Goal:** Keep simple memory in a chat-like program.

**Teaches Rust:**

- `Vec`
- pushing values
- iterating
- ownership inside collections
- formatting history

**Teaches AI:**

- conversation history
- context
- why memory matters

**Student builds:**

A mini terminal chat that stores previous user messages.

### Lesson 12 — Traits by Swapping AI Backends

**Goal:** Design code around behavior, not concrete names.

**Teaches Rust:**

- `trait`
- `impl`
- trait bounds or simple trait objects
- mock implementations

**Teaches AI:**

- model provider abstraction
- testing without calling a model
- clean architecture

**Student builds:**

A `LanguageModel` trait with an `OllamaModel` implementation and a `MockModel` for testing.

**Key teaching line:**

> Traits let your code depend on what something can do, not what it is called.

### Lesson 13 — Modules by Organizing the Ollama Client

**Goal:** Turn a single file into a clean small project.

**Teaches Rust:**

- `mod`
- `pub`
- file boundaries
- module imports

**Teaches AI:**

- separate client logic
- separate prompt templates
- separate task routing

**Student builds:**

```text
src/
├── main.rs
├── ollama.rs
├── prompts.rs
├── tasks.rs
└── errors.rs
```

## Phase 3 — Useful Local AI Tools

This phase turns Rust knowledge into tools students can actually use.

### Lesson 14 — AI File Summarizer

**Goal:** Summarize a local file with Ollama.

**Teaches Rust:**

- `std::fs`
- `PathBuf`
- command-line arguments
- reading text files

**Teaches AI:**

- document summarization
- prompt wrapping
- input cleaning

**Student builds:**

```bash
cargo run summarize ./article.txt
```

### Lesson 15 — Rust Code Explainer

**Goal:** Use Ollama to explain Rust source files.

**Teaches Rust:**

- reading source files
- multiline prompt strings
- formatting large prompts
- basic CLI ergonomics

**Teaches AI:**

- code explanation
- local coding assistant behavior
- prompt specificity

**Student builds:**

```bash
cargo run explain examples/ownership.rs
```

### Lesson 16 — Notes Q&A with Simple Retrieval

**Goal:** Build a simple RAG-like notes assistant without a vector database.

**Teaches Rust:**

- reading directories
- multiple files
- simple scoring
- document structs
- sorting results

**Teaches AI:**

- retrieval
- context injection
- grounded answers

**Student builds:**

```bash
cargo run ask "How does borrowing work?"
```

The program searches local notes, selects relevant context, and sends it to Ollama.

### Lesson 17 — Streaming Responses and Async Rust

**Goal:** Understand async through something visible: token streaming.

**Teaches Rust:**

- `async`
- `.await`
- `tokio`
- response streams
- incremental output

**Teaches AI:**

- streaming model responses
- better user experience
- perceived latency

**Student builds:**

A CLI where the answer appears gradually instead of all at once.

### Lesson 18 — Mini AI Agent Loop

**Goal:** Build the simplest useful agent loop.

**Teaches Rust:**

- loops
- structs
- enums
- JSON parsing
- safety limits
- error handling across steps

**Teaches AI:**

- Think → Act → Observe
- tool request
- observation
- stopping condition
- `MAX_ITERATIONS`

**Student builds:**

A local agent loop with a hard safety limit.

**Required safety rule:**

Every agent loop must include:

```rust
const MAX_ITERATIONS: usize = 10;
```

### Lesson 19 — Tools for the Agent

**Goal:** Give the local agent safe abilities.

**Teaches Rust:**

- command structs
- tool dispatch
- result handling
- modules
- enum-based tools or trait-based tools

**Teaches AI:**

- tool calling
- local automation
- tool boundaries
- safety

**Student builds:**

An agent with safe tools:

- read a file
- list a directory
- summarize a file
- ask Ollama

### Lesson 20 — Final Project: Local Rust AI Assistant

**Goal:** Combine the course into one useful assistant.

**Student builds:**

```bash
rust-ai-assistant ask "Explain this project"
rust-ai-assistant summarize README.md
rust-ai-assistant chat
rust-ai-assistant agent "Find TODOs and summarize them"
```

**Combines:**

- Ollama API
- Rust CLI
- structs
- enums
- traits
- modules
- error handling
- async
- file I/O
- simple retrieval
- mini agent loop

## Per-Lesson Deliverables

Each lesson should include:

- `README.md` lesson recap
- `index.html` student-facing lesson page
- `examples/` runnable Rust snippets
- intentional error files when the lesson teaches a compiler rule
- `quiz/` interactive HTML/CSS/JS quiz
- optional PDF recap
- source reference links

## Lesson Writing Template

Every lesson should follow this pattern:

1. **The useful thing we are building**
2. **Run the smallest possible version**
3. **Explain the Rust concept inside the code**
4. **Break it intentionally**
5. **Read the compiler or runtime error**
6. **Fix it**
7. **Make it slightly more useful**
8. **Recap the concept in plain language**
9. **Quiz**
10. **One small challenge**

## Quiz Rules

- Questions must be course-relevant and non-trivial.
- Distractors must be plausible Rust misunderstandings.
- Correct answers must be spread across A/B/C/D.
- Each question should teach something even when the learner answers wrong.
- AI-era lessons should test both Rust understanding and practical AI-tool understanding.

## Navigation Rules

When a lesson is added or changed:

- update root `index.html`
- update `site/index.html`
- update `README.md`
- update this roadmap if the lesson scope changes
- commit related files together

A lesson is not done if students cannot find it from the course hub.
