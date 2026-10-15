# rkv — Learning Milestones

This document describes the learning trajectory, not an implementation guide.

The AI tutor should introduce one milestone at a time and adapt the sequence to the student's progress.

## Phase 1 — Interactive CLI

**Milestone 1: Create a Rust application**

Create a Cargo project, compile it, and run it.

Concepts: Cargo, crates, main function, compilation.

**Milestone 2: Read user input**

Build a program that repeatedly reads lines from standard input.

Concepts: String, mutable variables, stdin, Result, loops.

**Milestone 3: Recognize commands**

Introduce simple textual commands and a way to exit the application.

Concepts: string slices, pattern matching, control flow.

## Phase 2 — In-memory storage

**Milestone 4: Implement SET and GET**

Store key-value pairs in memory and retrieve them.

Concepts: HashMap, String vs &str, ownership, allocation.

**Milestone 5: Implement DEL and EXISTS**

Support deletion and checking whether a key exists.

Concepts: mutable and immutable borrowing, Option, bool.

**Milestone 6: Validate command arguments**

Reject commands with incorrect argument counts.

Concepts: slices, indexing, defensive input handling.

## Phase 3 — Modeling the program

**Milestone 7: Introduce Command**

Represent operations using an enum rather than relying only on command strings.

Concepts: enums, associated data, pattern matching.

**Milestone 8: Separate parsing from execution**

Convert input tokens into structured commands before executing them.

Concepts: API design, borrowing, lifetimes.

**Milestone 9: Introduce Store**

Encapsulate the underlying collection behind a dedicated type.

Concepts: structs, impl blocks, methods, visibility.

**Milestone 10: Separate logic from presentation**

Remove terminal output from storage operations.

Concepts: return types, Option, ownership transfer, separation of concerns.

## Phase 4 — Error handling and architecture

**Milestone 11: Model parsing errors**

Distinguish unknown commands, invalid argument counts, and empty input.

Concepts: Result, error enums, structured errors.

**Milestone 12: Improve parser API**

Use slices rather than requiring a specific collection type.

Concepts: slices, deref coercion, API generality.

**Milestone 13: Handle EOF**

Exit correctly when standard input reaches end-of-file.

Concepts: I/O contracts, byte counts, EOF.

**Milestone 14: Separate command execution**

Move command execution out of the interactive input loop.

Concepts: ownership transfer, responsibilities.

**Milestone 15: Organize modules**

Separate command representation, parsing, storage, and the application entry point.

Concepts: modules, imports, privacy, visibility.

## Phase 5 — Testing

**Milestone 16: First unit test**

Verify that a stored value can be retrieved.

Concepts: unit tests, assertions, test modules.

**Milestone 17: Test Store behavior**

Cover insertion, replacement, deletion, existence, and missing keys.

Concepts: behavioral contracts, test isolation.

**Milestone 18: Test the parser**

Cover valid commands, invalid argument counts, unknown commands, and empty input.

Concepts: PartialEq, Debug, derived traits, Result assertions.

## Final review

Before completing the project, the student should be able to explain:

- Where heap allocations occur
- Who owns stored keys and values
- Why borrowed commands need lifetimes
- How Move, Copy, and Clone differ
- Why GET and DEL have different borrowing requirements
- How Option differs from Result
- Why parser and Store should remain independent
- What unit tests verify that the compiler cannot

## Completion

The project is complete when the application works, its core behavior is tested, and the student can explain its ownership and architectural decisions.

Do not add persistence, networking, concurrency, or complex abstractions simply to extend this project.
