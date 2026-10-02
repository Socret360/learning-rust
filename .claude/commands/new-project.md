---
description: Scaffold a book project, exercise, or DSA library crate and register it.
argument-hint: <name> | exercise/<name> | dsa/<name> | books/<book>/<name>
---

Scaffold a new project: $ARGUMENTS

The default book is `books/the-rust-programming-language`.

Choose the target by the argument prefix:

- `dsa/<name>` — a DSA library crate:
  1. Run `cargo new --lib dsa/<name>`.
  2. Append `dsa/<name>/Cargo.toml` to `rust-analyzer.linkedProjects` in
     `.vscode/settings.json`.
  3. Add a `#[cfg(test)] mod tests` block to `src/lib.rs` with one failing
     placeholder test, plus a `// TODO:` comment naming the structure or
     algorithm. Do not write the implementation.
- `exercise/<name>` — a standalone practice binary crate:
  1. Run `cargo new exercises/<name>`.
  2. Append `exercises/<name>/Cargo.toml` to `rust-analyzer.linkedProjects`.
  3. Leave `src/main.rs` as the default hello world plus a `// TODO:` comment
     naming the exercise. Do not write the solution.
- `books/<book>/<name>` — a project for a specific book:
  1. Run `cargo new books/<book>/<name>`.
  2. Append `books/<book>/<name>/Cargo.toml` to `rust-analyzer.linkedProjects`.
  3. Leave `src/main.rs` as the default hello world plus a `// TODO:` comment
     naming the section. Do not write learner code.
- anything else (a bare `<name>`) — a project for the current book,
  `books/the-rust-programming-language`:
  1. Run `cargo new books/the-rust-programming-language/<name>`.
  2. Append `books/the-rust-programming-language/<name>/Cargo.toml` to
     `rust-analyzer.linkedProjects`.
  3. Leave `src/main.rs` as the default hello world plus a `// TODO:` comment
     naming the book section. Do not write learner code.

In every case, show what changed and remind the learner to restart
rust-analyzer.
