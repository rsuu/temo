use rusqlite::{types::ToSqlOutput, Connection};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use tabled::{Style, Table, Tabled};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub uuid: Uuid,
    pub urgency: f32,

    pub author: String,
    pub name: String,
    pub progress: String,

    // #[tabled(display_with = "table_project")]
    pub project: Option<Uuid>,

    pub status: TaskStatus,
    pub date_info: DateInfo,

    // #[tabled(display_with = "table_history")]
    pub history: Option<History>,
    pub is_history: bool,
}

impl Tabled for Task {
    const LENGTH: usize = 80;
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "NAME".to_string(),
            "AUTHOR".to_string(),
            "PROGRESS".to_string(),
            "PROJ".to_string(),
            "STATUS".to_string(),
        ]
    }
    fn fields(&self) -> Vec<String> {
        let proj = if let Some(p) = &self.project {
            p.to_string()
        } else {
            "".to_string()
        };

        let _his = if let Some(h) = &self.history {
            serde_json::to_string(&h).expect("")
        } else {
            "".to_string()
        };

        vec![
            format!("{}", self.id),
            //  format!("{}", self.urgency),
            format!("{}", self.name),
            format!("{}", self.author),
            format!("{}", self.progress),
            proj,
            String::from(&self.status),
            //            String::from(&self.date_info),
            //            his,
            //           format!("{}", self.is_history),
        ]

        //         vec![
        //             format!("{}", self.id),
        //             format!("{}", self.uuid),
        //             format!("{}", self.urgency),
        //             format!("{}", self.author),
        //             format!("{}", self.name),
        //             format!("{}", self.progress),
        //             proj,
        //             String::from(&self.status),
        //             String::from(&self.date_info),
        //             his,
        //             format!("{}", self.is_history),
        //         ]
    }
}
fn table_project(proj: &Option<bool>) -> String {
    match proj {
        Some(v) => format!("is valid thing = {}", v),
        None => "is not valid".to_string(),
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct DateInfo {
    pub create_at: String,
    pub delete_at: Option<String>,
    pub edit_at: Option<String>,

    pub start_at: Option<String>,
    pub done_at: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Project {
    pub name: String,
    pub uuid: Uuid,
    pub members: Vec<Uuid>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum TaskStatus {
    Pending,
    Completed,
    Deleted,
    Recurring,
    Waiting,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum DateState {
    DateNotDue,
    DateAfterToday,
    DateLaterToday,
    DateEarlierToday,
    DateBeforeToday,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum ModType {
    ModReplace,
    ModPrepend,
    ModAppend,
    ModAnnotate,
}

impl Task {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            uuid: Uuid::new_v4(),
            urgency: 0.0,
            author: "".to_string(),
            name: name.to_string(),
            progress: "".to_string(),
            project: None,
            status: TaskStatus::Pending,
            date_info: DateInfo::new(),
            history: None,
            is_history: false,
        }
    }

    pub fn set_progress(&mut self, progress: &str) {
        self.progress = progress.to_string();
    }

    pub fn set_project(&mut self, name: &str, conn: &Connection) {
        // serarch name
        //   if has
        //     poj::link_to()
        //   else
        //     poj::new()
        if let Some(uuid) = Project::search(name, conn) {
            self.project = Some(uuid);
        } else if let Some(uuid) = Project::link() {
            self.project = Some(uuid);
        }
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_string();
    }

    pub fn cat(&self) {
        // |ID|St|UUID|A|Age|Done|Project|Description

        println!("{}", Table::new(&[&self]).with(Style::psql()));
    }
}

impl DateInfo {
    pub fn new() -> Self {
        Self {
            create_at: OffsetDateTime::now_utc().format(&Rfc3339).expect(""),
            delete_at: None,
            edit_at: None,
            start_at: None,
            done_at: None,
        }
    }

    pub fn edit_at(&mut self) {
        self.edit_at = Some(OffsetDateTime::now_utc().format(&Rfc3339).expect(""));
    }

    pub fn start_at(&mut self) {
        self.start_at = Some(OffsetDateTime::now_utc().format(&Rfc3339).expect(""));
    }

    pub fn done_at(&mut self) {
        self.done_at = Some(OffsetDateTime::now_utc().format(&Rfc3339).expect(""));
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            uuid: Uuid::new_v4(),
            members: vec![],
        }
    }

    pub fn push(&mut self, uuid: &Uuid) {
        self.members.push(*uuid)
    }

    pub fn link() -> Option<Uuid> {
        todo!()
    }

    pub fn search(name: &str, conn: &Connection) -> Option<Uuid> {
        conn.query_row(
            "SELECT value FROM Self WHERE name=$1",
            rusqlite::params![name],
            |row| row.get::<usize, String>(0),
        )
        .expect("");

        todo!()
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct History(Vec<Uuid>);

impl History {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: &Uuid) {
        self.push(value)
    }
}

impl rusqlite::ToSql for History {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(serde_json::to_string(self).unwrap()))
    }
}

impl TryFrom<&History> for String {
    type Error = ();
    fn try_from(s: &History) -> Result<Self, Self::Error> {
        if let Ok(v) = serde_json::to_string(&s) {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl From<&Project> for std::string::String {
    fn from(s: &Project) -> Self {
        serde_json::to_string(s).expect("")
        // impl option<project>
    }
}

impl From<&TaskStatus> for String {
    fn from(s: &TaskStatus) -> Self {
        use TaskStatus::*;

        match *s {
            Pending => "P".to_string(),
            Completed => "C".to_string(),
            Deleted => "D".to_string(),
            Recurring => "R".to_string(),
            Waiting => "W".to_string(),
        }
    }
}

impl From<&DateInfo> for std::string::String {
    fn from(s: &DateInfo) -> Self {
        serde_json::to_string(s).expect("")
    }
}

impl From<DateInfo> for std::string::String {
    fn from(s: DateInfo) -> Self {
        String::from(&s)
    }
}

impl From<&str> for DateInfo {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("")
    }
}

impl From<&str> for TaskStatus {
    fn from(s: &str) -> Self {
        use TaskStatus::*;

        match s {
            "P" => Pending,
            "C" => Completed,
            "D" => Deleted,
            "R" => Recurring,
            "W" => Waiting,
            _ => panic!(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|{}|{}|", self.id, self.name, self.status);
        Ok(())
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self).as_str());
        Ok(())
    }
}
