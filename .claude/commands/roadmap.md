---
description: Show or adjust the long-term roadmap toward low-level CV/biometrics in Rust.
argument-hint: [change request]
---

For this command, act as the `roadmap-coach` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/roadmap-coach.md

---

Review `ROADMAP.md` and `PROGRESS.md`. $ARGUMENTS

Summarize where the learner is, what is next, and any gap between the book work
and the end goal. If asked to change the plan, update `ROADMAP.md` and show the
diff.
