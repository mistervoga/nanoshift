# nanoshift

**nanoshift** is a simple, command-line based to-do list application to help you manage your tasks efficiently. Keep track of what needs to be done, organize your day, and boost your productivity with ease.

## Key Features

- Lightweight and Efficient: As the name suggests, nanoshift is designed to be a small, powerful tool that doesn't overwhelm you with unnecessary features. It's all about getting things done quickly and efficiently.
- Temporary Task Management: nanoshift embraces the idea of temporary and regularly resettable task lists. This encourages you to focus on immediate tasks and clear your list frequently, promoting a dynamic and adaptable workflow.
- CSV Export: Easily export your tasks to a CSV file, allowing you to keep a record of completed tasks, share your to-do list with others, or perform additional data analysis.
- Project Switching: Seamlessly switch between different projects, each with its own set of tasks, without losing focus on what's important.
- Reminders and Due Dates: Set due dates and reminders for your tasks to stay on track and ensure that nothing falls through the cracks.

## Features

- Add new tasks with a simple command.
- List all your pending and completed tasks.
- Mark tasks as completed.
- Delete individual tasks or clear all tasks at once.
- Switch between different projects.
- Export tasks to a CSV file for backup or sharing.

## Installation

1. Clone the repository to your local machine

```sh
    git clone https://github.com/mistervoga/nanoshift.git
    cd taskline
```

2. Compile the code

```sh
    cargo build --release
```

3. Run the executable

```sh
    ./target/release/nanoshift
```

4. Move executable to system-wide location with access to your Path

```sh
    # Assumes you have a bin folder in the home directory, alternatively you can move it to /usr/bin/nsh
    mv /target/release/nanoshift ~/bin/nsh
```

## Usage

```sh
# Initialize a new project
nsh init [project_name]

# Add a new task
nsh add "Task description"

# List all tasks
nsh list

# Mark a task as completed
nsh complete <task_index>

# Delete a task
nsh delete <task_index>

# Delete all tasks
nsh delete -a

# Switch to a different project
nsh switch <project_name>

# Export tasks to a CSV file
nsh export [project_name]

# List all projects
nsh projects
```
