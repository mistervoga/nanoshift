# nanoshift

**nanoshift** is a minimal, offline-first CLI task manager built for clarity and speed.  
No accounts. No sync. No noise. Just tasks and focus.

This repo is called **nanoshift** — the installed binary is **`shift`** (to avoid collisions with `ns` on Linux).

---

## Features

- Ultra-lightweight CLI
- Global scope + project scopes
- Fast context switching
- Bulk deletion for clean resets
- CSV export
- Markdown export
- “Today” + “Focus” views
- Local SQLite storage (no cloud)

---

## Installation

```bash
git clone https://github.com/mistervoga/nanoshift.git
cd nanoshift
cargo build --release
```

Binary:
```
target/release/shift
```

Optional global install:
```bash
cargo install --path . --force
```

---

## Usage

Initialize DB:
```bash
shift init
```

Add a task:
```bash
shift add "Buy milk"
```

List tasks:
```bash
shift list
```

Complete / delete:
```bash
shift complete 1
shift delete 1
```

Delete all tasks in current scope:
```bash
shift delete-all
```

Switch scope:
```bash
shift switch work
shift switch global
```

Show projects:
```bash
shift projects
```

Status (scope + counts):
```bash
shift status
```

Today (open tasks only):
```bash
shift today
```

Focus (minimal view):
```bash
shift focus
```

Export CSV:
```bash
shift export
shift export tasks.csv
```

Export Markdown:
```bash
shift export-md
shift export-md notes.md
```

---

## Data Storage

Nanoshift stores everything locally using SQLite.

### Linux
```
~/.local/share/nanoshift/tasks.db
```

### macOS
```
~/Library/Application Support/nanoshift/tasks.db
```

### Windows
```
%APPDATA%\nanoshift\tasks.db
```

---

## License

MIT (see `LICENSE` file).
