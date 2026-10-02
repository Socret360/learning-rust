---
description: Generate an algorithm problem with complexity constraints, tests, and a hidden solution.
argument-hint: <topic>
---

For this command, act as the `dsa-coach` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/dsa-coach.md

---

Give the learner an algorithm problem: $ARGUMENTS

Calibrate to their DSA tier (read `PROGRESS.md` and `ROADMAP.md`). Include:

- problem statement,
- constraints (input size, target time/space complexity),
- 2-3 examples including edge cases,
- a function signature,
- escalating hints in `<details>` blocks,
- a solution with tests in a `<details>` block.

Prefer problems that map to the CV end goal (spatial search, graph segmentation,
DP on images, string matching). Ask them to state the complexity of their
approach before they code. Do not create files.
