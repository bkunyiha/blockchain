<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../ci/kubernetes/README.md) - Kubernetes production guide
22. **Chapter 10: Rust Language Guide** ← *You are here*

</details>

</div>

---

# Rust installation and local setup

In this chapter, we set up Rust on a local machine so we can build, run, and debug the code in this repository. Our goal is pragmatic: by the end, we can run `cargo build`, `cargo test`, `cargo fmt`, and `cargo clippy`, and we know where the important Rust project files live.

## What “done” looks like

Before we move on to language features, we want a working baseline:

- **Toolchain**: `rustc` and `cargo` are installed and on `PATH`
- **Formatting**: `cargo fmt` runs successfully
- **Linting**: `cargo clippy` runs successfully
- **Project build**: the `bitcoin/` crate builds and tests pass locally

## Quickstart (copy/paste)

If Rust is already installed, this block is the fastest path to a clean baseline for this repository. We run a build/test cycle first, then run formatting and linting to catch issues early.

```bash
# From the repository root:
cd bitcoin

# Build + test
cargo build
cargo test

# Format + lint
cargo fmt
cargo clippy
```

## Install Rust (rustup)

We install Rust using `rustup`, which is the official toolchain manager. It installs `rustc` (the compiler) and `cargo` (the build tool), and it also lets us add components like `clippy` and `rustfmt`.

### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Make `cargo` available in this shell session:
source "$HOME/.cargo/env"
```

### Windows (PowerShell)

On Windows, `winget` installs rustup system-wide:

```powershell
winget install --id Rustlang.Rustup -e
```

### Verify

These commands confirm the compiler and toolchain manager are installed and discoverable via `PATH`:

```bash
rustc --version
cargo --version
rustup --version
```

## Official online references (bookmark these)

- **Rust + rustup**: [Install Rust](https://www.rust-lang.org/tools/install)
- **The Rust Book**: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- **Cargo**: [The Cargo Book](https://doc.rust-lang.org/cargo/)
- **rustfmt**: [rustfmt](https://github.com/rust-lang/rustfmt)
- **Clippy**: [Clippy](https://github.com/rust-lang/rust-clippy)
- **rust-analyzer**: [rust-analyzer](https://rust-analyzer.github.io/)
- **VS Code extension**: [rust-analyzer (VS Code Marketplace)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- **IntelliJ/CLion plugin**: [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust)

If we are on a team, it is common to standardize on the **stable** toolchain:

This makes sure “it works on my machine” means “it works on the team’s machine”:

```bash
rustup default stable
rustup update
```

## Cargo: package manager + build tool

Cargo is Rust’s standard workflow tool. It builds code, runs tests, downloads dependencies, and generates documentation.

- **Build**: `cargo build` (debug), `cargo build --release`
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Docs**: `cargo doc --open`

In Rust projects, `Cargo.toml` declares dependencies and metadata. `Cargo.lock` pins dependency versions for reproducible builds (commonly committed for binaries).

## Formatting and linting: rustfmt + clippy

Rust projects typically use two standard tools to keep the codebase readable and maintainable:

- **`rustfmt`**: the official Rust formatter. It rewrites code into a consistent style so we do not waste time in code review debating formatting.
- **`clippy`**: the official Rust linter. It flags suspicious patterns and common mistakes (performance footguns, needless allocations, unintuitive code) and often suggests the more idiomatic alternative.

In day-to-day work, we run them like this:

```bash
cargo fmt
cargo clippy
```

If `cargo fmt` or `cargo clippy` are missing, we can add them to the toolchain explicitly:

```bash
rustup component add rustfmt
rustup component add clippy
```

If we want stricter CI-like checking locally, we can also run:

```bash
cargo fmt --check
```

## IDE integration (rust-analyzer)

We get the best experience by using an editor with `rust-analyzer`, which provides IDE features backed by the compiler’s understanding of the code (types, traits, macro expansion, etc.).

- **rust-analyzer (project docs)**: the official documentation site for features, configuration, and troubleshooting.  
  Link: [rust-analyzer](https://rust-analyzer.github.io/) (`https://rust-analyzer.github.io/`)
- **VS Code (extension install page)**: install/enable the extension so VS Code can power “go to definition”, inline errors, and code actions for Rust.  
  Link: [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) (`https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer`)
- **IntelliJ / CLion (Rust plugin page)**: install/enable JetBrains’ Rust plugin for IDE support (code navigation, inspections, run configurations).  
  Link: [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust) (`https://plugins.jetbrains.com/plugin/8182-rust`)

Once enabled, we should see:

- jump-to-definition and symbol search
- in-editor compiler diagnostics
- inline type hints
- test discovery and run buttons

## Hello world (sanity-check the toolchain)

We can verify our local toolchain with a small project. `cargo new` creates a tiny crate, and `cargo run` builds + runs it.

```bash
cargo new hello-rust
cd hello-rust
cargo run
```

## What the key files/directories mean

If you just ran `cargo new hello-rust` above, the folder that command created is a concrete example of a “typical Rust crate layout”. In most Rust crates, we will see:

```text
hello-rust/
  Cargo.toml
  src/
    main.rs
  # (later you may also see: tests/, benches/, examples/, and target/)
```

- **`Cargo.toml`**: crate metadata + dependencies
- **`src/`**: source code
  - `src/main.rs`: binary entry point (when the crate builds an executable)
  - `src/lib.rs`: library entry point (when the crate exports a library API)
- **`tests/`**: integration tests
- **`target/`**: build outputs (generated; safe to delete if we need a clean rebuild)

In this repository, the Rust implementation we walk through lives under `bitcoin/` (it contains its own `Cargo.toml` and `src/`).

## Build this repository’s Rust project

From the repository root, we enter the `bitcoin/` crate and build/test it. This ensures dependencies resolve and the project compiles on our machine.

```bash
cd bitcoin
cargo build
cargo test
```

If we are iterating quickly, it also helps to keep a lint loop handy:

This catches formatting drift and common mistakes early (before a larger refactor makes them harder to spot):

```bash
cargo fmt
cargo clippy
```

## Common setup issues (quick fixes)

- **`command not found: cargo`**: the `rustup` install script typically adds Cargo to your shell profile; restart the shell or ensure `$HOME/.cargo/bin` is on `PATH`.
- **Clippy/fmt missing**: run `rustup component add clippy rustfmt`.
- **Build works in one terminal but not another**: different shells can load different profiles; check `echo $PATH`.

---

## Next steps

Continue to **[Introduction →](01-Introduction.md)**.

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Rust installation and local setup** | **[Introduction →](01-Introduction.md)** 📚

</div>

---

*In this chapter, we set up Rust and our local development tooling. Continue to [Introduction](01-Introduction.md) to learn Rust’s design philosophy and how we use it in this project.*


