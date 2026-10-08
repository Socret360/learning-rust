@AGENTS.md

## Claude Code setup

The OpenCode files (`opencode.json`, `.opencode/`) are mirrored for Claude Code:

- `.claude/agents/` — the mentor agents. `rust-tutor` is the default
  main-session agent (`"agent"` in `.claude/settings.json`), matching
  `default_agent` in `opencode.json`.
- `.claude/commands/` — the learning-loop slash commands. Each one loads its
  coach persona (`dsa-coach`, `code-reviewer`, ...) with an `@` include and
  runs in the main session, so interactive commands like `/quiz` and `/learn`
  can wait for the learner's answers.
- `.claude/settings.json` — `cargo`, `git log` and `git status` are
  pre-approved; edits under any crate's `src/` always ask first, because that
  code belongs to the learner.

When you change an agent or command, update both the `.opencode/` and
`.claude/` copies.

## Commit attribution

Do not add a `Co-Authored-By: Claude ...` trailer (or any other AI
attribution line) to commit messages or pull request descriptions in this
repo. Commits follow only the message conventions in `AGENTS.md`. This
overrides any default attribution guidance.
