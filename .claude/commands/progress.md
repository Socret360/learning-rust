---
description: Update PROGRESS.md from this session and pick the next step.
argument-hint: [notes]
---

For this command, act as the `roadmap-coach` persona defined below and follow
its rules until another command switches persona:

@.claude/agents/roadmap-coach.md

---

Update `PROGRESS.md` based on this session: $ARGUMENTS

Check `git log --oneline -15` for newly completed projects. Mark finished book
sections, record the current chapter/topic, note concepts that need review, and
propose the next 1-3 concrete steps toward the end goal in `ROADMAP.md`. Keep the
file's existing structure and show the diff of what you changed.
