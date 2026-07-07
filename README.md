# Rust for the AI Era

**Learn Rust by building useful local AI tools with Ollama.**

A beginner-friendly Rust course by **Ayman Hamed** for developers who want Rust to feel practical in the AI era.

This is not a traditional Rust course. The first lessons build a small Rust survival kit, then the course pivots into Ollama-powered projects: local prompts, JSON requests, error handling, file summarizers, code explainers, simple retrieval, streaming responses, and a mini AI agent loop.

The teaching strategy is simple but impactful: build a small useful example, run it, explain the Rust concept inside it, show what can fail, then improve it gradually.

## Public Lesson Hub

When GitHub Pages is enabled, the public course hub will be available at:

<https://ayman3000.github.io/rust-for-the-ai-era/>

Lesson 00:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/00_install_and_first_program/>

Lesson 00 quiz:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/00_install_and_first_program/quiz/>

Lesson 01 quiz:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/01_data_types/quiz/>

Lesson 02:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/02_variable_types/>

Lesson 02 quiz:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/02_variable_types/quiz/>

Lesson 03:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/03_functions_for_ai_utilities/>

Lesson 03 quiz:

<https://ayman3000.github.io/rust-for-the-ai-era/lessons/03_functions_for_ai_utilities/quiz/>

## Learning Flow

Every lesson follows the same system:

1. **Watch the video**
2. **Read the recap**
3. **Run the code**
4. **Try the interactive quiz**
5. **Continue only when the concept feels clear**

## Lessons

| # | Lesson | Concept | Quiz |
|---|--------|---------|------|
| 00 | [Install Rust & First Program](lessons/00_install_and_first_program/) | `fn main`, `let`, immutability, `mut`, `println!` | [Open quiz](lessons/00_install_and_first_program/quiz/) |
| 01 | [Data Types](lessons/01_data_types/) | Scalar and compound types | [Open quiz](lessons/01_data_types/quiz/) |
| 02 | [Variable Types](lessons/02_variable_types/) | Inference, annotations, `mut`, shadowing, constants | [Open quiz](lessons/02_variable_types/quiz/) |
| 03 | [Functions for AI Utilities](lessons/03_functions_for_ai_utilities/) | parameters, return values, expressions, simple AI utility helpers without borrowing | [Open quiz](lessons/03_functions_for_ai_utilities/quiz/) |
| 04 | Control Flow for Prompt Decisions | `if`, loops, beginner `match`, prompt validation | planned |
| 05 | Ownership: Who Owns the Data? | move, clone, scope, `String`, drop | planned |
| 06 | Borrowing: Passing Prompts Without Copies | references, `&T`, `&mut T`, borrowing rules | planned |
| 07 | First Ollama Request from Rust | Cargo crates, `reqwest`, `serde_json`, local inference | planned |
| 08 | Structs by Modeling Ollama JSON | request/response structs, `Serialize`, `Deserialize` | planned |
| 09 | Error Handling When Ollama Fails | `Result`, `?`, readable runtime errors | planned |
| 10 | Enums by Creating AI Task Types | task variants, `match`, prompt routing | planned |
| 11 | Collections by Building Prompt History | `Vec`, chat history, context | planned |
| 12 | Traits by Swapping AI Backends | `trait`, `impl`, Ollama vs mock model | planned |
| 13 | Modules by Organizing the Ollama Client | `mod`, `pub`, project structure | planned |
| 14 | AI File Summarizer | file I/O, CLI arguments, local summarization | planned |
| 15 | Rust Code Explainer | source reading, multiline prompts, local coding assistant | planned |
| 16 | Notes Q&A with Simple Retrieval | directories, scoring, context injection | planned |
| 17 | Streaming Responses and Async Rust | `async`, `.await`, token streaming | planned |
| 18 | Mini AI Agent Loop | Think → Act → Observe, safety limit | planned |
| 19 | Tools for the Agent | safe local tools, dispatch, observations | planned |
| 20 | Final Project: Local Rust AI Assistant | complete Ollama-powered CLI assistant | planned |

## Course Promise

Rust can feel strict at first. This course makes that strictness understandable. We learn one concept at a time, run small examples, read compiler feedback, and build a strong mental model.

## Repository Structure

```text
lessons/       Lesson folders with examples, recaps, and quizzes
docs/          Course roadmap and publishing notes
site/          Future static course hub / GitHub Pages entry
assets/        Shared images, diagrams, and branding assets
```

## Instructor

**Ayman Hamed** — AI Engineer, Course Creator, YouTuber.
