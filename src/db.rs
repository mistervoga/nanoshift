use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection, project: Option<&str>) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL UNIQUE
                  )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
                  id INTEGER PRIMARY KEY,
                  description TEXT NOT NULL,
                  completed INTEGER NOT NULL,
                  date_added TEXT NOT NULL,
                  date_completed TEXT,
                  project_id INTEGER,
                  FOREIGN KEY(project_id) REFERENCES projects(id)
                  )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS current_project (
                  id INTEGER PRIMARY KEY,
                  project_id INTEGER,
                  FOREIGN KEY(project_id) REFERENCES projects(id)
                  )",
        [],
    )?;

    if let Some(proj) = project {
        conn.execute(
            "INSERT INTO projects (name) VALUES (?1)
             ON CONFLICT(name) DO NOTHING",
            params![proj],
        )?;

        let project_id: i32 = conn.query_row(
            "SELECT id FROM projects WHERE name = ?1",
            params![proj],
            |row| row.get(0),
        )?;

        conn.execute(
            "INSERT INTO current_project (id, project_id) VALUES (1, ?1)
             ON CONFLICT(id) DO UPDATE SET project_id=excluded.project_id",
            params![project_id],
        )?;
        println!("Project '{}' initialized and set as current project.", proj);
    } else {
        println!("Global To-Do file initialized.");
    }

    Ok(())
}

pub fn set_project(conn: &Connection, project: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO projects (name) VALUES (?1)
         ON CONFLICT(name) DO NOTHING",
        params![project],
    )?;

    let project_id: i32 = conn.query_row(
        "SELECT id FROM projects WHERE name = ?1",
        params![project],
        |row| row.get(0),
    )?;

    conn.execute(
        "INSERT INTO current_project (id, project_id) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET project_id=excluded.project_id",
        params![project_id],
    )?;
    println!("Switched to project '{}'.", project);
    Ok(())
}

pub fn get_current_project(conn: &Connection) -> Result<Option<String>> {
    let mut stmt = conn.prepare(
        "SELECT p.name FROM current_project cp
         JOIN projects p ON cp.project_id = p.id
         WHERE cp.id = 1"
    )?;
    let project_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    })?;

    for project in project_iter {
        return Ok(Some(project?));
    }

    Ok(None)
}
