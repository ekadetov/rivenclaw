# rivenclaw

![rivenclaw banner](assets/banner.png)

Learn Rust and agentic coding by building a Claude Code sibling from scratch.

No magic. No black boxes. Just the raw plumbing -- built live, one vertical slice at a time.

Each slice is guided by Claude Code. The meta-twist: Claude Code builds its own Rust sibling.

---

## What You Are Building

A working AI coding assistant in Rust -- rivenclaw is the Rust sibling of Claude Code.
By the end you will have a CLI that takes a prompt, talks to the Anthropic API,
runs tools (read, write, bash, glob, grep), manages a context window, enforces
permissions, fires hooks, and can spawn sub-agents.

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

## What You Will Learn

**Rust fundamentals:**
- Ownership, borrowing, and lifetimes in a real async codebase
- Trait objects for a pluggable tool interface
- `Result` / `?` error propagation with `anyhow`
- `tokio` async runtime and streaming with Server-Sent Events
- `serde_json` for API request/response serialization
- `clap` derive macros for CLI argument parsing
- `rstest` for parameterized and fixture-based tests

**Agentic systems:**
- How an agent loop works: stream tokens, detect tool calls, execute, loop
- Concurrent vs serial tool batching decisions
- Context window management and message compaction
- Permission rule evaluation (allow / deny / ask)
- Hook execution: PreToolUse, PostToolUse, Stop
- Spawning and orchestrating sub-agents

---

## Slices + Videos

| # | Feature | Status |
|---|---------|--------|
| 00 | Project scaffold + Cargo layout | done |
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
cargo build
cargo test
./target/debug/rivenclaw "explain this codebase"
```

---

## Stack

`tokio` -- `clap` -- `serde_json` -- `anyhow` -- `rstest`

---

## Contributing

Open an issue. Ship a PR. All skill levels welcome -- follow the slice you are on.
