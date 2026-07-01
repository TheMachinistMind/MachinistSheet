# MachinistSheet

A CLI tool that snapshots your entire engineering mind: code projects, architecture, and knowledge vaults in one command.

---

## What is this?

Most developers track their code on GitHub. But GitHub only shows you files and commits.
MachinistSheet shows you the full picture — what you built, what changed, and what you know.

Run it once. Get a snapshot of your entire project: recent commits, active endpoints, source files, and your Obsidian notes — all in one place.

---

## How it works

1. Create a `devsheet.toml` config file (see example below)
2. Add your projects and knowledge vaults
3. Run `cargo run -- /path/to/your/project`
4. Get a full snapshot in your terminal

---

## Setup

Clone the repo and build it:

```bash
git clone https://github.com/TheMachinistMind/MachinistSheet.git
cd MachinistSheet
cargo build
```

Copy the example config and fill in your paths:

```bash
cp devsheet.example.toml devsheet.toml
```

---

## Configuration

MachinistSheet reads from a `devsheet.toml` file. Here's what it looks like:

```toml
[[projects]]
name = "My Project"
path = "/path/to/your/project"

[[vaults]]
name = "Second Brain"
path = "/path/to/your/obsidian/vault"
```

---

## What it shows

For each project:
- Last 5 git commits
- Active API endpoints
- Source files

For each vault:
- All notes

---

## Why I built this

I'm building a real-time multiplayer app while learning Rust from scratch.
My main project got too complex to learn from efficiently.
So I built a smaller tool on the side — same language, simpler scope, faster feedback.

MachinistSheet started as a way to document my build progress for social media.
It became something more: a daily snapshot of how my engineering mind is growing.

---

## Status

Early stage. Works for Rust + React projects with Axum backends.
Obsidian vault support is basic — more coming.

Built with Rust. No dependencies except `toml` and `serde`.
