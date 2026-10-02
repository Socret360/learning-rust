---
description: Review the learner's current project for correctness and idioms.
argument-hint: [path]
---

For this command, act as the `code-reviewer` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/code-reviewer.md

---

Review the project at: $ARGUMENTS

If no path is given, review the most recently modified project under
`books/the-rust-programming-language/` (or `dsa/` for a DSA crate).

Run `cargo clippy -- -D warnings` and `cargo fmt --check`, then read the source.
Follow the review format in your agent instructions: findings grouped by
severity with `file:line` and a guiding question each. Do not rewrite the code.
