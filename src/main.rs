mod db;
mod tasks;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shift")]
#[command(about = "Nanoshift — minimal, focused task management.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Ensure database + schema exist
    Init,

    /// Add a task to the current scope
    Add { task: String },

    /// List tasks in current scope
    List,

    /// Mark task as completed by id
    Complete { id: i64 },

    /// Delete task by id
    Delete { id: i64 },

    /// Delete all tasks in current scope
    DeleteAll,

    /// Show all projects (always includes 'global')
    Projects,

    /// Switch active scope (project name or 'global')
    Switch { project: String },

    /// Export tasks in current scope to CSV (default: nanoshift_export.csv)
    Export { path: Option<String> },

    /// Show scope + open/done counts
    Status,

    /// Show only open tasks (simple “today” view)
    Today,

    /// Minimal “focus” view (open tasks only, bullet list)
    Focus,

    /// Export tasks in current scope to Markdown (default: nanoshift.md)
    ExportMd { path: Option<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = db::connect_and_init()?;

    match cli.command {
        Command::Init => {
            println!("shift ready");
        }

        Command::Add { task } => {
            tasks::add_task(&conn, &task)?;
        }

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

        Command::Complete { id } => {
            tasks::complete_task(&conn, id)?;
        }

        Command::Delete { id } => {
            tasks::delete_task(&conn, id)?;
        }

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
            let (open, done) = tasks::counts_in_scope(&conn)?;
            println!("scope: {}", scope.as_str());
            println!("open:  {}", open);
            println!("done:  {}", done);
        }

        Command::Today => {
            for t in tasks::list_open_tasks(&conn)? {
                println!("{:<4} {}", t.id, t.description);
            }
        }

        Command::Focus => {
            println!("--- focus ---");
            for t in tasks::list_open_tasks(&conn)? {
                println!("• {}", t.description);
            }
        }

        Command::ExportMd { path } => {
            let path = path.unwrap_or_else(|| "nanoshift.md".into());
            let n = tasks::export_scope_md(&conn, &path)?;
            println!("exported {} -> {}", n, path);
        }
    }

    Ok(())
}
