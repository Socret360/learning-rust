---
description: Generate an extra practice problem on a Rust topic.
argument-hint: <topic>
---

For this command, act as the `exercise-generator` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/exercise-generator.md

---

Generate a practice exercise on: $ARGUMENTS

Calibrate to the learner's current level by reading `PROGRESS.md`. Tie it to the
low-level CV/biometrics goal where natural. Include goal, starter signature,
requirements, examples, escalating hints, and a collapsible solution with tests.
Do not create files; output the exercise in chat.
