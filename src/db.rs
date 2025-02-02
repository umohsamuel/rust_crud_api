use crate::models::{Task, User};
use rusqlite::{Connection, Result};
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn create_task(&self, title: &str, completed: bool) -> Result<Task> {
        let id = Uuid::new_v4();
        self.conn.execute(
            "INSERT INTO tasks (id, title, completed) VALUES (?1, ?2, ?3)",
            rusqlite::params![id.to_string(), title, completed],
        )?;
        // let id = self.conn.last_insert_rowid() as i32;
        Ok(Task {
            id,
            title: title.to_string(),
            completed,
        })
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed FROM tasks")?;
        let tasks = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                Ok(Task {
                    id: Uuid::parse_str(&id_str).unwrap(),
                    title: row.get(1)?,
                    completed: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(tasks)
    }

    pub fn update_task(&self, id: Uuid, completed: bool, title: &str) -> Result<Task> {
        self.conn.execute(
            "UPDATE tasks SET title = ?1, completed = ?2 WHERE id = ?3",
            rusqlite::params![title, completed, id.to_string()],
        )?;

        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed FROM tasks WHERE id = ?1")?;

        let task = stmt.query_row([id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            Ok(Task {
                id: Uuid::parse_str(&id_str).unwrap(),
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        Ok(task)
    }

    pub fn delete_task(&self, id: Uuid) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", [id.to_string()])?;
        Ok(())
    }

    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<User> {
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        self.conn.execute(
            "INSERT INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)",
            rusqlite::params![id_str, username, password_hash],
        )?;

        let mut stmt = self
            .conn
            .prepare("SELECT id, username, password_hash, created_at FROM users WHERE id = ?1")?;
        let user = stmt.query_row(rusqlite::params![id_str], |row| {
            let id_str: String = row.get(0)?;
            Ok(User {
                id: Uuid::parse_str(&id_str).unwrap(),
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3).ok(),
            })
        })?;
        Ok(user)
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<User> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = ?1",
        )?;
        let user = stmt.query_row(rusqlite::params![username], |row| {
            let id_str: String = row.get(0)?;
            Ok(User {
                id: Uuid::parse_str(&id_str).unwrap(),
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3).ok(),
            })
        })?;
        Ok(user)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(rusqlite::params![key])?;
        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
