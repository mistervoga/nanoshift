# nanoshift

**nanoshift** is a minimal, offline-first CLI task manager built for clarity and speed.  
No accounts. No sync. No noise. Just tasks and focus.

This repo is called **nanoshift** — the installed binary is **`nsh`** (to avoid collisions with `ns` on Linux).

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
target/release/nsh
```

Optional global install:
```bash
cargo install --path . --force
```

---

## Usage

Initialize DB:
```bash
nsh init
```

Add a task:
```bash
nsh add "Buy milk"
```

List tasks:
```bash
nsh list
```

Complete / delete:
```bash
nsh complete 1
nsh delete 1
```

Delete all tasks in current scope:
```bash
nsh delete-all
```

Switch scope:
```bash
nsh switch work
nsh switch global
```

Show projects:
```bash
nsh projects
```

Status (scope + counts):
```bash
nsh status
```

Today (open tasks only):
```bash
nsh today
```

Focus (minimal view):
```bash
nsh focus
```

Export CSV:
```bash
nsh export
nsh export tasks.csv
```

Export Markdown:
```bash
nsh export-md
nsh export-md notes.md
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
