<div align="center">

# 🏺 JARS - Just Another Rust Shell

*A shell written in Rust. Just a project to explore systems programming and build something usable.*

![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/status-work%20in%20progress-yellow?style=flat-square)
![License](https://img.shields.io/badge/license-do%20whatever-blue?style=flat-square)

</div>

---

## 🚀 Installation

> Requires a recent stable Rust toolchain.

```sh
git clone https://github.com/Shr3ne/just-another-rust-shell
cd just-another-rust-shell
cargo build --release
```

Run it with `cargo run`.

---

## Features

| Feature | Description |
|---|---|
| 🔧 **Command execution** | Runs external programs by searching `PATH` |
| 📦 **Built-ins** | `cd`, `exit`, and other shell essentials handled internally |
| 📜 **Command history** | Navigate previous commands with arrow keys; persists across sessions |
| ✏️ **Line editing** | Cursor movement, home/end, and editing shortcuts via [reedline](https://github.com/nushell/reedline) |
| 💬 **Custom prompt** | Shows current working directory |

---

## Todo

- [ x ] Pipelines &nbsp;`cmd1 | cmd2`
- [ ] Tab autocomplete for commands and paths
- [ ] I/O redirection &nbsp;`>` &nbsp;`>>` &nbsp;`<`
- [ ] Environment variable expansion &nbsp;`$VAR`
- [ ] Shell scripting / running script files
- [ ] Syntax highlighting in the prompt
- [ ] Command aliases

---

## 📦 Dependencies

| Crate | Purpose |
|---|---|
| [reedline](https://crates.io/crates/reedline) | Line editing and history |

---

<div align="center">
<sub>do whatever you want with it</sub>
</div>
