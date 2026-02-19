// src/tasks.rs

use anyhow::{bail, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    Project(String),
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::Global => "global",
            Scope::Project(name) => name.as_str(),
        }
    }

    pub fn parse(input: &str) -> Scope {
        let s = input.trim();
        if s.eq_ignore_ascii_case("global") {
            Scope::Global
        } else {
            Scope::Project(s.to_string())
        }
    }
}

pub fn get_scope(conn: &Connection) -> Result<Scope> {
    let raw: String = conn.query_row(
        "SELECT value FROM meta WHERE key='scope'",
        [],
        |row| row.get(0),
    )?;
    Ok(Scope::parse(&raw))
}

pub fn set_scope(conn: &Connection, project: &str) -> Result<()> {
    let scope = Scope::parse(project);

    if let Scope::Project(ref name) = scope {
        if name.trim().is_empty() {
            bail!("project name cannot be empty");
        }
        // ensure project exists
        conn.execute(
            "INSERT OR IGNORE INTO projects(name) VALUES(?1)",
            params![name],
        )?;
    }

    conn.execute(
        "UPDATE meta SET value=?1 WHERE key='scope'",
        params![scope.as_str()],
    )?;

    Ok(())
}

pub fn list_projects(conn: &Connection) -> Result<Vec<String>> {
    let mut out = vec!["global".to_string()];

    let mut stmt = conn.prepare("SELECT name FROM projects ORDER BY name ASC")?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;

    for r in rows {
        out.push(r?);
    }

    Ok(out)
}

fn current_project_id(conn: &Connection) -> Result<Option<i64>> {
    match get_scope(conn)? {
        Scope::Global => Ok(None),
        Scope::Project(name) => Ok(conn
            .query_row(
                "SELECT id FROM projects WHERE name=?1",
                params![name],
                |row| row.get(0),
            )
            .optional()?),
    }
}

pub fn add_task(conn: &Connection, description: &str) -> Result<()> {
    let desc = description.trim();
    if desc.is_empty() {
        bail!("task cannot be empty");
    }

    // if scope is project and missing, create it (safety)
    let scope = get_scope(conn)?;
    let pid = match scope {
        Scope::Global => None,
        Scope::Project(ref name) => {
            conn.execute(
                "INSERT OR IGNORE INTO projects(name) VALUES(?1)",
                params![name],
            )?;
            Some(conn.query_row(
                "SELECT id FROM projects WHERE name=?1",
                params![name],
                |row| row.get(0),
            )?)
        }
    };

    conn.execute(
        "INSERT INTO tasks(description, completed, created_at, project_id)
         VALUES(?1, 0, ?2, ?3)",
        params![desc, Utc::now().to_rfc3339(), pid],
    )?;
    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let pid = current_project_id(conn)?;
    let mut out = vec![];

    if let Some(pid) = pid {
        let mut stmt = conn.prepare(
            "SELECT id, description, completed
             FROM tasks
             WHERE project_id=?1
             ORDER BY completed ASC, id ASC",
        )?;

        let rows = stmt.query_map(params![pid], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i64>(2)? == 1,
            })
        })?;

        for r in rows {
            out.push(r?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, description, completed
             FROM tasks
             WHERE project_id IS NULL
             ORDER BY completed ASC, id ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i64>(2)? == 1,
            })
        })?;

        for r in rows {
            out.push(r?);
        }
    }

    Ok(out)
}

pub fn complete_task(conn: &Connection, id: i64) -> Result<()> {
    if conn.execute("UPDATE tasks SET completed=1 WHERE id=?1", params![id])? == 0 {
        bail!("no such task id");
    }
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<()> {
    if conn.execute("DELETE FROM tasks WHERE id=?1", params![id])? == 0 {
        bail!("no such task id");
    }
    Ok(())
}

pub fn delete_all_in_scope(conn: &Connection) -> Result<usize> {
    let pid = current_project_id(conn)?;
    Ok(if let Some(pid) = pid {
        conn.execute("DELETE FROM tasks WHERE project_id=?1", params![pid])?
    } else {
        conn.execute("DELETE FROM tasks WHERE project_id IS NULL", [])?
    })
}

pub fn export_scope_csv(conn: &Connection, path: &str) -> Result<usize> {
    let tasks = list_tasks(conn)?;
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(["id", "completed", "description"])?;

    for t in &tasks {
        wtr.write_record([
            t.id.to_string(),
            (if t.completed { "1" } else { "0" }).to_string(),
            t.description.clone(),
        ])?;
    }

    wtr.flush()?;
    Ok(tasks.len())
}
