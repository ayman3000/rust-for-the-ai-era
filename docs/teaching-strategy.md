# Rust for the AI Era — Teaching Strategy

## The Identity

This course teaches Rust through useful local AI tools.

It is not a textbook-style Rust course. It is a builder course:

> Learn the Rust concept, use it immediately, and keep building toward a local AI assistant.

## The Formula

Every lesson follows Ayman's teaching strategy:

```text
Simple concept → tiny runnable example → practical AI-era use → clear explanation → small challenge
```

The course should feel gradual, not overwhelming.

The student should never feel:

> Why am I learning this?

Instead, each Rust concept should answer a practical question:

- Why functions? To reuse prompt-cleaning code.
- Why ownership? To understand who owns prompt text.
- Why borrowing? To pass prompts without copying.
- Why structs? To model Ollama JSON.
- Why enums? To represent AI task types.
- Why error handling? Because local AI services fail.
- Why traits? To swap AI backends cleanly.
- Why modules? To organize the Ollama client.
- Why async? To stream model responses.

## The Turning Point

Lessons 00–06 are the Rust survival kit.

After borrowing, the course should pivot into AI tooling:

```text
Lesson 07: First Ollama request from Rust
```

This is the moment the course becomes unique.

## Lesson Design Rules

### 1. One Concept Per Lesson

Do not combine too many Rust concepts in one lesson.

A beginner should be able to say:

> Today I learned exactly one important idea.

### 2. Code First, Explain Second

Start with working code as early as possible.

Then explain what happened.

### 3. Use Practical Examples

Prefer examples that could become part of a real tool:

- prompt cleaner
- token estimate helper
- Ollama request
- file summarizer
- code explainer
- notes Q&A
- mini agent loop

Avoid abstract examples unless they are very short and immediately followed by a useful version.

### 4. Show Failure

Rust teaches through errors. Use that.

Each concept lesson should include at least one intentional mistake when useful:

- immutable variable changed
- value moved then reused
- wrong type parsed
- missing Ollama server
- model not found
- invalid JSON shape

### 5. Keep the AI Examples Local

Core examples should work with:

- Rust
- Cargo
- Ollama
- a local model

No paid API key in the core path.

## Tone

The tone should be beginner-friendly, practical, and confident.

Use plain language:

- "Rust is strict because it is protecting you."
- "The compiler is not your enemy. It is your first reviewer."
- "We are not memorizing syntax. We are building tools."
- "If the code runs, we can improve it."

Avoid unnecessary academic language.

## Course Promise

By the end, the student builds a local Rust AI assistant that can:

- ask Ollama questions
- summarize files
- explain Rust code
- keep simple chat history
- retrieve notes
- stream responses
- run a small safe agent loop

The final message of the course:

> You did not just learn Rust. You used Rust to build AI tools.
