use rusqlite::{params, Connection, Result, OptionalExtension};
use chrono::prelude::*;
use std::fs::File;
use std::error::Error;
use csv::Writer;

pub fn add_task(conn: &Connection, task: &str) -> Result<()> {
    let project_name = crate::db::get_current_project(conn)?.unwrap_or_else(|| "Global".to_string());

    let project_id: Option<i32> = if project_name.eq_ignore_ascii_case("global") {
        None
    } else {
        conn.query_row(
            "SELECT id FROM projects WHERE name = ?1",
            params![project_name],
            |row| row.get(0)
        ).optional()?
    };

    let now: DateTime<Local> = Local::now();
    conn.execute(
        "INSERT INTO tasks (description, completed, date_added, date_completed, project_id) VALUES (?1, 0, ?2, NULL, ?3)",
        params![task, now.format("%Y-%m-%d %H:%M:%S").to_string(), project_id],
    )?;
    println!("Task added: {}", task);
    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<()> {
    let current_project = crate::db::get_current_project(conn)?;
    let query = match &current_project {
        Some(proj) if !proj.eq_ignore_ascii_case("global") => {
            "SELECT t.id, t.description, t.completed, t.date_added, t.date_completed, p.name FROM tasks t
             JOIN projects p ON t.project_id = p.id WHERE p.name = ?1"
        }
        _ => {
            "SELECT t.id, t.description, t.completed, t.date_added, t.date_completed, p.name FROM tasks t
             LEFT JOIN projects p ON t.project_id = p.id WHERE t.project_id IS NULL"
        }
    };

    let mut stmt = conn.prepare(query)?;
    let task_iter: Box<dyn Iterator<Item = Result<(_, _, _, _, _, _)>>> = match &current_project {
        Some(proj) if !proj.eq_ignore_ascii_case("global") => {
            Box::new(stmt.query_map(params![proj], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, bool>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            })?)
        }
        _ => {
            Box::new(stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, bool>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            })?)
        }
    };

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

pub fn complete_task(conn: &Connection, id: i32) -> Result<()> {
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

pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
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

pub fn delete_all_tasks(conn: &Connection) -> Result<()> {
    let current_project = crate::db::get_current_project(conn)?;
    let query = match current_project {
        Some(ref proj) => format!("DELETE FROM tasks WHERE project_id = (SELECT id FROM projects WHERE name = '{}')", proj),
        None => "DELETE FROM tasks WHERE project_id IS NULL".to_string(),
    };

    conn.execute(&query, [])?;
    println!("All tasks have been deleted.");
    Ok(())
}

pub fn export_project(conn: &Connection, project_name: &str) -> Result<(), Box<dyn Error>> {
    let query = format!(
        "SELECT id, description, completed, date_added, date_completed FROM tasks WHERE project_id = (SELECT id FROM projects WHERE name = '{}')",
        project_name
    );

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
