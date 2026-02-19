// src/main.rs

mod db;
mod tasks;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tasks::{Scope};

#[derive(Parser)]
#[command(name = "nanoshift")]
#[command(about = "Minimal, focused task management.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Add { task: String },
    List,
    Complete { id: i64 },
    Delete { id: i64 },
    DeleteAll,
    Projects,
    Switch { project: String },
    Export { path: Option<String> },
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = db::connect_and_init()?;

    match cli.command {
        Command::Init => println!("nanoshift ready"),
        Command::Add { task } => tasks::add_task(&conn, &task)?,
        Command::List => {
            let items = tasks::list_tasks(&conn)?;
            if items.is_empty() {
                println!("no tasks");
            } else {
                for t in items {
                    let mark = if t.completed { "✓" } else { " " };
                    println!("{:<4} [{}] {}", t.id, mark, t.description);
                }
            }
        }
        Command::Complete { id } => tasks::complete_task(&conn, id)?,
        Command::Delete { id } => tasks::delete_task(&conn, id)?,
        Command::DeleteAll => {
            let n = tasks::delete_all_in_scope(&conn)?;
            println!("deleted {}", n);
        }
        Command::Projects => {
            for p in tasks::list_projects(&conn)? {
                println!("{}", p);
            }
        }
        Command::Switch { project } => {
            tasks::set_scope(&conn, &project)?;
            println!("scope: {}", tasks::get_scope(&conn)?.as_str());
        }
        Command::Export { path } => {
            let path = path.unwrap_or_else(|| "nanoshift_export.csv".into());
            let n = tasks::export_scope_csv(&conn, &path)?;
            println!("exported {} -> {}", n, path);
        }
        Command::Status => {
            let scope = tasks::get_scope(&conn)?;
            match scope {
                Scope::Global => println!("scope: global"),
                Scope::Project(name) => println!("scope: {}", name),
            }
        }
    }

    Ok(())
}
