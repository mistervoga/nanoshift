mod db;
mod tasks;

use rusqlite::Connection;
use std::env;
use std::path::PathBuf;
use std::result::Result;

use db::{init_db, set_project, list_projects};
use tasks::{add_task, list_tasks, complete_task, delete_task, delete_all_tasks, export_project};

fn get_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Failed to get executable directory")?;
    Ok(exe_dir.join("tasks.db"))
}

fn print_usage() {
    println!("taskline 0.1");
    println!("taskline is a task management CLI tool to help you organize your projects and tasks efficiently.");
    println!("Author: Nicolas von Garrel <mistervoga@gmail.com>");
    println!();
    println!("Usage:");
    println!("  tl init [project_name]      Initializes a new project");
    println!("  tl add <task>               Adds a new task to the current project");
    println!("  tl list                     Lists all tasks in the current project");
    println!("  tl complete <task_index>    Marks a task as completed");
    println!("  tl delete <task_index>      Deletes a task");
    println!("  tl delete -a                Deletes all tasks in the current project");
    println!("  tl switch <project_name>    Switches to a different project");
    println!("  tl export [project_name]    Exports tasks of the current or specified project to a CSV file");
    println!("  tl projects                 Lists all projects");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let db_path = get_db_path()?;
    let conn = Connection::open(db_path)?;

    match args[1].as_str() {
        "init" => {
            if args.len() < 3 {
                init_db(&conn, None)?;
            } else {
                init_db(&conn, Some(&args[2]))?;
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("Error: Specify a task to add.");
                print_usage();
            } else {
                add_task(&conn, &args[2])?;
            }
        }
        "list" => {
            list_tasks(&conn)?;
        }
        "complete" => {
            if args.len() < 3 {
                println!("Error: Specify the ID of the task to complete.");
                print_usage();
            } else {
                let id = args[2].parse::<i32>().expect("Invalid ID");
                complete_task(&conn, id)?;
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Error: Specify the ID of the task to delete.");
                print_usage();
            } else if args[2] == "-a" {
                delete_all_tasks(&conn)?;
            } else {
                let id = args[2].parse::<i32>().expect("Invalid ID");
                delete_task(&conn, id)?;
            }
        }
        "switch" => {
            if args.len() < 3 {
                println!("Error: Specify the project name to switch to.");
                print_usage();
            } else {
                set_project(&conn, &args[2])?;
            }
        }
        "export" => {
            if args.len() < 3 {
                export_project(&conn, "Global").expect("Failed to export Global project.");
            } else {
                export_project(&conn, &args[2]).expect("Failed to export project.");
            }
        }
        "projects" => {
            list_projects(&conn)?;
        }
        _ => {
            println!("Error: Invalid command.");
            print_usage();
        }
    }

    Ok(())
}
