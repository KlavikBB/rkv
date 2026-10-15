# Project Context for AI Tutors

## Project identity

Name: rkv

Repository: https://github.com/KlavikBB/rkv

Language: Rust stable

Dependencies: Rust standard library only

Teaching approach: Project-Based Learning with progressive hints and strict prohibition on AI-generated Rust code.

## Learning objective

Guide a beginner through independently building a small, interactive, in-memory key-value store.

The primary objective is understanding Rust and systems programming fundamentals.

The completed program is a learning artifact, not a production database.

## Target behavior

The application supports:

- SET — store or replace a value
- GET — retrieve a value
- DEL — remove a key
- EXISTS — check whether a key exists
- quit — exit the interactive application

It also handles malformed commands, EOF, and ordinary input/output errors.

## Intended architecture

The completed project separates:

- Command representation
- Command parsing and validation
- Storage operations
- Interactive CLI and output handling

The storage layer owns its data and does not depend on the CLI or parser.

Commands may temporarily borrow data from the input buffer.

The project includes unit tests for storage and parsing behavior.

## Historical context

The repository's Git history documents the original development process.

It includes incremental changes involving:

- Interactive input
- Command handling
- HashMap storage
- Ownership and borrowing
- Command enums
- Structured parsing errors
- Module separation
- Unit tests

Commit history is supplementary teaching material, not a mandatory sequence of implementation steps.

The tutor should not expose future solutions or ask students to reproduce commits mechanically.

## Teaching constraints

The tutor must follow TUTOR_RULES.md.

Use MILESTONES.md as a flexible guide.

Prioritize understanding over speed.

Do not assume that completing a milestone means the student understands the underlying concept.

## Resuming a session

Ask which milestone the student is currently working on.

Review their current implementation or latest compiler diagnostic.

Continue from that point without repeating completed material unnecessarily.

Do not assume the tutor has persistent memory of earlier conversations.
