// Task
// task info

// Db
// check database
// update uuid after sync
// is_sync uuid
// bool    Uuid

use crate::data::task;
use rusqlite::{params, Connection};
use uuid::Uuid;

const TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS Task (
            uuid         TEXT NOT NULL,
            id           INTEGER NOT NULL,
            urgency      REAL,

            author       TEXT NOT NULL,
            name         TEXT NOT NULL,
            progress     TEXT NOT NULL,
            project      JSON1,

            status       TEXT,
            date_info    TEXT,

            history      JSON1,
            is_history   BOOL
    )
"#;

const TABLE_SELF: &str = r#"
    CREATE TABLE IF NOT EXISTS Self (
            version      INT,
            uuid         TEXT NOT NULL,
            lock         BOOL,
            last_id      INT,
            drop_id      TEXT
    )
"#;

#[derive(Debug)]
pub struct Db {
    pub conn: Connection,
}

impl Db {
    // task
    pub fn new_table(&self) {
        self.conn.execute(TABLE, []).expect("");
    }

    pub fn insert(&mut self, task: &task::Task) {
        self.conn
            .execute(
                "INSERT INTO Task (
            uuid,
            id,
            urgency,

            author,
            name,
            progress,
            project,

            status,
            date_info,

            history,
            is_history

            )
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                params![
                    task.uuid.to_string(),
                    task.id,
                    task.urgency,
                    task.author,
                    task.name,
                    task.progress,
                    if let Some(v) = task.project {
                        v.to_string()
                    } else {
                        "".to_string()
                    },
                    String::from(&task.status).as_str(),
                    String::from(&task.date_info).as_str(),
                    task.history,
                    task.is_history,
                ],
            )
            .expect("");
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.conn
            .execute(
                "DELETE FROM Self WHERE uuid = ?1;",
                params![uuid.to_string()],
            )
            .expect("");
    }

    pub fn update(&mut self, uuid: &Uuid) {
        match self
            .conn
            .execute("UPDATE Task SET uuid = ?1", params![uuid.to_string()])
        {
            Ok(s) => eprintln!("Updated: {}", s),
            Err(e) => eprintln!("{}", e),
        }
    }

    pub fn search(&self) {}

    // self
    pub fn dbself_new_table(&self) {
        if check_table_exists(&self.conn, "Self") {
        } else {
            self.conn.execute(TABLE_SELF, []).expect("");
            Self::dbself_insert(self, Uuid::new_v4(), false, 0, "");
        }
    }

    pub fn dbself_insert(&self, uuid: Uuid, lock: bool, last: usize, drop: &str) {
        let _ = self
            .conn
            .execute(
                "INSERT INTO Self (
            version,
            uuid,
            lock,
            last_id,
            drop_id
            )
        VALUES (?1,?2,?3,?4,?5)",
                params![0, uuid.to_string(), lock, last, drop],
            )
            .expect("");
    }

    pub fn dbself_remove(&self, uuid: &Uuid) {
        let _ = self
            .conn
            .execute(
                "DELETE FROM Self WHERE uuid = ?1;",
                params![uuid.to_string()],
            )
            .expect("");
    }

    pub fn dbself_set_last(&self, last: usize) {
        //println!("{}", get_col_count(&self.conn));

        self.conn
            .execute(
                "UPDATE Self SET last_id = ?1 WHERE version=0",
                params![last],
            )
            .expect("");
    }

    pub fn dbself_set_uuid(&self, uuid: &Uuid) {
        self.conn
            .execute(
                "UPDATE Self SET uuid = ?1 WHERE version=0",
                params![uuid.to_string()],
            )
            .expect("");
    }
}

pub fn check_table_exists(conn: &Connection, table_name: &str) -> bool {
    let sql: &str = r#"
        SELECT COUNT(`name`)
        FROM `sqlite_master`

        WHERE `type` = 'table'
            AND `name` = ?"#;

    let mut stmt = conn.prepare(sql).unwrap();
    let count = stmt
        .query_row(params![table_name], |row| {
            row.get(0) as rusqlite::Result<i32>
        })
        .expect("");

    println!("{}", count);
    count > 0
}

pub fn get_col_count(conn: &Connection) -> usize {
    let sql: &str = r#"
SELECT COUNT(*) AS CNTREC FROM pragma_table_info('Self') WHERE name='version'
"#;

    let mut stmt = conn.prepare(sql).unwrap();
    let count = stmt
        .query_row([], |row| {
            row.get(0) as rusqlite::Result<usize>
        })
        .expect("");

    println!("{}", count);
    count
}
