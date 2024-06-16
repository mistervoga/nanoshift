use rusqlite::{params, Connection};
use chrono::{Local, NaiveDateTime};

pub fn send_reminders(conn: &Connection) -> rusqlite::Result<()> {
    let now = Local::now().naive_local();
    let mut stmt = conn.prepare("SELECT id, description, due_date, reminder FROM tasks WHERE reminder IS NOT NULL AND reminder <= ?1 AND completed = 0")?;
    let task_iter = stmt.query_map(params![now], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<NaiveDateTime>>(2)?,
            row.get::<_, Option<NaiveDateTime>>(3)?,
        ))
    })?;

    for task in task_iter {
        let (id, description, due_date, reminder): (i32, String, Option<NaiveDateTime>, Option<NaiveDateTime>) = task?;
        println!("Reminder: Task '{}' (ID: {}) is due soon!", description, id);
        if let Some(due) = due_date {
            println!("Due Date: {}", due);
        }
    }
    Ok(())
}
