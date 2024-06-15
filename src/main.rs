mod db;
mod tasks;

use db::{init_db, set_project, get_current_project};
use tasks::{add_task, list_tasks, complete_task, delete_task, delete_all_tasks, export_project};
use rusqlite::Connection;
use std::env;
use std::result::Result;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
