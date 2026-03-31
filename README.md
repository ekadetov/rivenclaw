# rivenclaw

> A Rust implementation of an AI coding assistant harness -- tool wiring, agent loop, and context management.

---

## What this is

rivenclaw is a minimal, auditable AI coding assistant runtime written in Rust. It focuses on the core plumbing: how tools are declared and executed, how an agent loop is orchestrated, and how context is managed across a session.

## Repository Layout

```text
.
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── agent/             # Agent loop and orchestration
│   ├── tools/             # Tool implementations (read, write, bash, glob, grep)
│   ├── harness/           # Permission model and hook system
│   └── context/           # Context window and message history
├── tests/                 # Integration and unit tests
├── Cargo.toml
└── README.md
```

## Status

Early stage. Tool wiring and the core agent loop are the current focus.

## Quickstart

```bash
# Build
cargo build --release

# Run
./target/release/rivenclaw "explain this codebase"

# Test
cargo test
```

## Contributing

Contributions are welcome. Open an issue or submit a PR -- all skill levels encouraged.

Areas that need help:
- Tool implementations
- Test coverage
- Documentation
