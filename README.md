# rivenclaw

**Build an AI coding assistant from scratch. In Rust.**

No Python. No magic. Just Rust, tokio, and the raw plumbing of an AI agent -- built live, slice by slice, with YouTube videos at every step.

---

## 🦀 Why This Exists

Most AI tooling is a black box. rivenclaw tears it open.

You will build every layer yourself:

```
  prompt
    |
    v
+----------+     +----------+     +----------+
|  context |---->|  agent   |---->|  tools   |
|  window  |     |   loop   |     | read/bash|
+----------+     +----------+     +----------+
                      |
                      v
               +----------+
               | harness  |  <-- permissions + hooks
               +----------+
                      |
                      v
              Anthropic API
```

Each box is a Rust module. Each module ships in a slice. Each slice has a video.

---

## 🎬 Follow Along

| Slice | Feature                  | Video |
|-------|--------------------------|-------|
| 00    | Project scaffold         | soon  |
| 01    | CLI + clap               | soon  |
| 02    | Tool trait + Read tool   | soon  |
| 03    | Context window           | soon  |
| 04    | Anthropic API call       | soon  |
| 05    | Agent loop               | soon  |
| 06    | Permission harness       | soon  |

---

## 🚀 Quickstart

```bash
cargo build --release
./target/release/rivenclaw "explain this codebase"
cargo test
```

---

## 💡 What You Will Learn

- Rust async with tokio
- Trait objects for pluggable tools
- Ownership inside a stateful agent loop
- Context window and message history management
- Calling the Anthropic API from Rust

---

## 🤝 Contributing

Open an issue. Submit a PR. All skill levels welcome.
