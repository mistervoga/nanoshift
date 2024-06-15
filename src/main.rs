use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::error::Error;
use csv::Writer;

fn print_usage() {
    println!("Usage:");
    println!("  taskline init [project_name]");
    println!("  taskline add <task>");
    println!("  taskline list");
    println!("  taskline complete <task_index>");
    println!("  taskline delete <task_index>");
    println!("  taskline delete -a");
    println!("  taskline switch <project_name>");
    println!("  taskline export <project_name>");
}

fn init_db(conn: &Connection, project: Option<&str>) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
                  id INTEGER PRIMARY KEY,
                  description TEXT NOT NULL,
                  completed INTEGER NOT NULL,
                  date_added TEXT NOT NULL,
                  date_completed TEXT,
                  project TEXT
                  )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS current_project (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL
                  )",
        [],
    )?;

    if let Some(proj) = project {
        conn.execute(
            "INSERT INTO current_project (id, name) VALUES (1, ?1)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name",
            params![proj],
        )?;
        println!("Project '{}' initialized and set as current project.", proj);
    } else {
        println!("Global To-Do file initialized.");
    }

    Ok(())
}

fn set_project(conn: &Connection, project: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO current_project (id, name) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET name=excluded.name",
        params![project],
    )?;
    println!("Switched to project '{}'.", project);
    Ok(())
}

fn get_current_project(conn: &Connection) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT name FROM current_project WHERE id = 1")?;
    let project_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    })?;

    for project in project_iter {
        return Ok(Some(project?));
    }

    Ok(None)
}

fn add_task(conn: &Connection, task: &str, project: Option<&str>) -> Result<()> {
    let now: DateTime<Local> = Local::now();
    conn.execute(
        "INSERT INTO tasks (description, completed, date_added, date_completed, project) VALUES (?1, 0, ?2, NULL, ?3)",
        params![task, now.format("%Y-%m-%d %H:%M:%S").to_string(), project],
    )?;
    println!("Task added: {}", task);
    Ok(())
}

fn list_tasks(conn: &Connection) -> Result<()> {
    let current_project = get_current_project(conn)?;
    let query = match current_project {
        Some(ref proj) => format!("SELECT id, description, completed, date_added, date_completed, project FROM tasks WHERE project = '{}'", proj),
        None => "SELECT id, description, completed, date_added, date_completed, project FROM tasks WHERE project IS NULL".to_string(),
    };

    let mut stmt = conn.prepare(&query)?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, Option<String>>(5)?,
        ))
    })?;

    for task in task_iter {
        let (id, description, completed, date_added, date_completed, project) = task?;
        let status = if completed {
            format!("(Completed on {})", date_completed.unwrap_or_default())
        } else {
            "(Pending)".to_string()
        };
        let project_info = match project {
            Some(proj) => format!(" [Project: {}]", proj),
            None => String::from(""),
        };
        println!("{}: {} {} [Added on {}]{}", id, description, status, date_added, project_info);
    }
    Ok(())
}

fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    let now: DateTime<Local> = Local::now();
    let changes = conn.execute(
        "UPDATE tasks SET completed = 1, date_completed = ?1 WHERE id = ?2",
        params![now.format("%Y-%m-%d %H:%M:%S").to_string(), id],
    )?;
    if changes == 0 {
        println!("Error: Invalid task ID");
    } else {
        println!("Task {} marked as completed", id);
    }
    Ok(())
}

fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    let changes = conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        params![id],
    )?;
    if changes == 0 {
        println!("Error: Invalid task ID");
    } else {
        println!("Task {} deleted", id);
    }
    Ok(())
}

fn delete_all_tasks(conn: &Connection) -> Result<()> {
    let current_project = get_current_project(conn)?;
    let query = match current_project {
        Some(ref proj) => format!("DELETE FROM tasks WHERE project = '{}'", proj),
        None => "DELETE FROM tasks WHERE project IS NULL".to_string(),
    };

    conn.execute(&query, [])?;
    println!("All tasks have been deleted.");
    Ok(())
}

fn export_project(conn: &Connection, project_name: &str) -> Result<(), Box<dyn Error>> {
    let query = format!("SELECT id, description, completed, date_added, date_completed FROM tasks WHERE project = '{}'", project_name);

    let mut stmt = conn.prepare(&query)?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
        ))
    })?;

    let file = File::create(format!("{}.csv", project_name))?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&["ID", "Description", "Completed", "Date Added", "Date Completed"])?;

    for task in task_iter {
        let (id, description, completed, date_added, date_completed) = task?;
        wtr.write_record(&[
            id.to_string(),
            description,
            completed.to_string(),
            date_added,
            date_completed.unwrap_or_default(),
        ])?;
    }

    wtr.flush()?;
    println!("Project '{}' exported to '{}.csv'.", project_name, project_name);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let conn = Connection::open("tasks.db")?;

    match args[1].as_str() {
        "init" => {
            if args.len() < 3 {
                init_db(&conn, None)?;
            } else {
                init_db(&conn, Some(&args[2]))?;
            }
        }
        "switch" => {
            if args.len() < 3 {
                println!("Error: Specify the project name to switch to.");
            } else {
                set_project(&conn, &args[2])?;
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("Error: Specify a task to add.");
            } else {
                let current_project = get_current_project(&conn)?;
                add_task(&conn, &args[2], current_project.as_deref())?;
            }
        }
        "list" => {
            list_tasks(&conn)?;
        }
        "complete" => {
            if args.len() < 3 {
                println!("Error: Specify the ID of the task to complete.");
            } else {
                let id = args[2].parse::<i32>().expect("Invalid ID");
                complete_task(&conn, id)?;
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Error: Specify the ID of the task to delete.");
            } else if args[2] == "-a" {
                delete_all_tasks(&conn)?;
            } else {
                let id = args[2].parse::<i32>().expect("Invalid ID");
                delete_task(&conn, id)?;
            }
        }
        "export" => {
            if args.len() < 3 {
                println!("Error: Specify the project name for export.");
            } else {
                export_project(&conn, &args[2]).expect("Failed to export project.");
            }
        }
        _ => {
            println!("Error: Invalid command.");
            print_usage();
        }
    }

    Ok(())
}