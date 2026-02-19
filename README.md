# nanoshift

**nanoshift** is a minimal, offline-first CLI task manager built for clarity and speed.  
No accounts. No sync. No noise. Just tasks and focus.

Nanoshift is designed for people who want a fast, temporary, resettable system that stays out of the way.

---

## Philosophy

Nanoshift is built around a few simple ideas:

- Minimal surface area → fewer decisions, more action  
- Temporary lists → tasks are meant to be cleared regularly  
- Offline-first → your data is yours  
- Project scopes → focus without complexity  

It’s not a productivity suite.  
It’s a sharp tool.

---

## Features

- Ultra-lightweight CLI
- Global scope + project scopes
- Fast context switching
- Bulk deletion for clean resets
- CSV export
- Local SQLite storage (no cloud)

---

## Concepts

### Scope
Nanoshift always operates in a **scope**:

- `global` (default)
- any named project

Switching scope changes what tasks you see.

---

## Installation

### 1. Clone the repo
```bash
git clone https://github.com/mistervoga/nanoshift.git
cd nanoshift
```

### 2. Build
```bash
cargo build --release
```

Binary will be here:
```
target/release/nanoshift
```

---

### 3. Optional: install globally

#### Linux / macOS
```bash
cargo install --path .
```

Or manual:
```bash
mv target/release/nanoshift ~/bin/nanoshift
```

---

## Usage

### Initialize database
```bash
nanoshift init
```

---

### Add task
```bash
nanoshift add "Buy milk"
```

---

### List tasks
```bash
nanoshift list
```

Example output:
```
1    [ ] Buy milk
2    [✓] Send email
```

---

### Complete task
```bash
nanoshift complete 1
```

---

### Delete task
```bash
nanoshift delete 1
```

---

### Delete all tasks (current scope)
```bash
nanoshift delete-all
```

---

### Switch scope

Create or switch project:
```bash
nanoshift switch work
```

Back to global:
```bash
nanoshift switch global
```

---

### Show projects
```bash
nanoshift projects
```

Always includes:
```
global
```

---

### Show current scope
```bash
nanoshift status
```

---

### Export to CSV
```bash
nanoshift export
```

Custom filename:
```bash
nanoshift export tasks.csv
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

## Updating

After pulling changes:

```bash
cargo install --path . --force
```

---

## License
MIT License
