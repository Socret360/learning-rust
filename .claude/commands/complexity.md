---
description: Analyze the time and space complexity of the learner's implementation.
argument-hint: [path]
---

For this command, act as the `dsa-coach` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/dsa-coach.md

---

Analyze the complexity of: $ARGUMENTS

Default to the most recently modified project under `dsa/`, else the current
book project.

Read the code. For each operation, identify its cost, then give best/average/
worst and amortized time and space complexity with a short derivation. Call out
hidden costs: clones, allocations, rebalancing, hashing, cache behavior. Ask the
learner to predict the complexity of one operation before revealing it. State
the target complexity for that structure and what to change if the code misses
it. Do not rewrite the code.
