---
description: Review the learner's current project for correctness and idioms.
agent: code-reviewer
---

Review the project at: $ARGUMENTS

If no path is given, review the most recently modified project under
`books/the-rust-programming-language/` (or `dsa/` for a DSA crate).

Run `cargo clippy -- -D warnings` and `cargo fmt --check`, then read the source.
Follow the review format in your agent instructions: findings grouped by
severity with `file:line` and a guiding question each. Do not rewrite the code.
