# rivenclaw

**Build an AI coding assistant from scratch. In Rust.**

No magic. No black boxes. Just the raw plumbing -- built live, one slice at a time, with a YouTube video at every step.

---

## The Stack

```
  your prompt
      |
      v
+----------+    +-------------------+    +------------------------+
|  context |    |    agent loop     |    |        tools           |
|  window  |--->|  stream -> tools  |--->|  read / write / bash   |
| compact  |    |  -> stream -> ... |    |  glob / grep / agent   |
+----------+    +-------------------+    +------------------------+
                        |
                        v
               +-------------------+
               |     harness       |
               |  permissions      |
               |  hooks (pre/post) |
               +-------------------+
                        |
                        v
               Anthropic Streaming API
```

---

## Slices + Videos

| # | Feature | Video |
|---|---------|-------|
| 00 | Project scaffold + Cargo layout | coming |
| 01 | CLI with clap | coming |
| 02 | Tool trait + Read / Write / Edit | coming |
| 03 | Bash tool (timeout, safety) | coming |
| 04 | Glob + Grep tools | coming |
| 05 | Anthropic API + streaming | coming |
| 06 | Agent loop (the query cycle) | coming |
| 07 | Permission harness + allow/deny rules | coming |
| 08 | Hook system (PreToolUse / PostToolUse / Stop) | coming |
| 09 | Context window + auto-compact | coming |
| 10 | Sub-agents (spawn + orchestrate) | coming |
| 11 | MCP client integration | coming |

---

## Quickstart

```bash
cargo build --release
./target/release/rivenclaw "explain this codebase"
cargo test
```

---

## What You Will Learn

- Streaming API responses with tokio async generators
- Trait objects for a pluggable tool interface
- Concurrent vs serial tool batching in an agent loop
- Borrow-safe context window and message compaction
- Permission rule evaluation (allow / deny / ask)
- Hook execution: shell, prompt, and http hooks
- Spawning and orchestrating sub-agents from Rust

---

## Stack

`tokio` -- `clap` -- `serde_json` -- `anyhow` -- `rstest`

---

## Contributing

Open an issue. Ship a PR. All skill levels welcome -- follow the slice you are on.
