// 获取参数
//
// 解析参数

use crate::{
    cli::{args, filter::Filter},
    data::{
        config, db,
        task::{self, History},
    },
    sync::{server},
};
use lexopt::prelude::*;
use rusqlite::{Connection};
use std::str::FromStr;
use uuid::Uuid;

pub async fn parse_args(config: &config::Config) -> Result<(), lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let conn = Connection::open(config.misc.db_path.as_str()).expect("");
    //let conn = Connection::open("./temo.db").expect("");
    //println!("{:?}", conn);

    let mut db = db::Db { conn };
    let _ = db.new_table();
    let _ = db.dbself_new_table();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                eprint_help();
            }

            Long("sync") => {
                let val = parser.value()?.into_string()?;

                match val.as_str() {
                    "s" | "server" => {
                        server::run_server().await;
                    }
                    "c" | "client" => {}
                    "d" | "daemon" => {
                        //daemon::run_daemon().await;
                    }
                    _ => {
                        safe_exit_with("");
                    }
                }
            }

            Long("add") => {
                let val = parser.value()?.into_string()?;

                //println!("{:?}",val);

                /// if has namedspace
                // TODO
                /// else get last_id
                let last_id = get_last_id(&db.conn);

                /// create a new task
                let mut task = args::create::Add::add(last_id + 1, val.as_str());
                let mut vals: Vec<String> = Vec::new();

                if let Ok(f) = parser.values() {
                    for j in f {
                        vals.push(j.to_string_lossy().to_string());
                    }

                    match vals.len() {
                        1 => {
                            task.set_progress(vals[0].as_str());
                        }

                        2 => {
                            task.set_progress(vals[0].as_str());
                            task.set_project(vals[1].as_str(), &db.conn);
                        }

                        3 => {
                            task.set_progress(vals[0].as_str());
                            task.set_project(vals[1].as_str(), &db.conn);
                            task.set_author(vals[2].as_str());
                        }
                        _ => {
                            eprintln!("Not support");
                            std::process::exit(-1);
                        }
                    }
                } else {
                }

                let _ = db.dbself_set_last(task.id);
                let _ = db.insert(&task);
                //println!("{:#?}", task);

                task.cat();
            }

            // parse subcommand
            Value(v) => {
                let val = v.into_string()?;
                let mut need_run: SubCmd = SubCmd::Null;

                // parse command
                if let Some(ids) = Filter::filter(val.as_str()) {
                    match ids.len() {
                        1 => {
                            // matching subcommands
                            if let Ok(sub) = parser.value() {
                                let subcommand = sub.into_string()?;

                                need_run = match match_sub(&subcommand) {
                                    "delete" => SubCmd::Delete(ids[0]),
                                    "ls" => SubCmd::Ls(ids[0]),
                                    _ => {
                                        unreachable!()
                                    }
                                };
                            } else {
                                // doing nothing
                            }
                            println!("{}", ids[0]);
                        }

                        _ => {
                            todo!()
                        }
                    }
                } else {
                }

                // run command
                match need_run {
                    SubCmd::Edit(_id) => {
                        todo!()
                    }
                    SubCmd::Delete(id) => {
                        // TODO
                        // delete a task by id
                        // move id to #Self.drop_id
                        println!("delete {}", id);

                        safe_exit_with("DONE");
                    }

                    SubCmd::Ls(id) => {
                        let tasks = get_task(&db.conn, &[id]).expect("");

                        for task in tasks.iter() {
                            task.cat();
                        }
                        //println!("{:#?}", tasks);
                    }
                    _ => {
                        eprint_help();
                        return Err(arg.unexpected());
                    }
                }
            }
            _ => {
                eprint_help();
                return Err(arg.unexpected());
            }
        }
    }

    Ok(())
}

enum ArgsCmd {
    Add(String, Option<String>, Option<String>), // title , description , project
}

enum SubCmd {
    Delete(usize), // te 1 drop
    Edit(usize),   //
    Ls(usize),
    Null,
}

// write to db
// read from db [id,project]

pub fn get_task(conn: &Connection, v: &[usize]) -> rusqlite::Result<Vec<task::Task>> {
    let mut res = Vec::new();

    let mut stmt = conn.prepare(
        "SELECT
            id          ,
            uuid        ,
            urgency     ,

            author      ,
            name        ,
            progress    ,
            project     ,

            status      ,
            date_info   ,

            history     ,
            is_history
            FROM Task
            WHERE id=:id;",
    )?;

    for id in v.iter() {
        let iter = stmt.query_map(&[(":id", id.to_string().as_str())], |row| {
            Ok(task::Task {
                id: row.get(0)?,
                uuid: Uuid::from_str(row.get::<usize, String>(1)?.as_str()).expect(""),
                urgency: row.get(2)?,

                author: row.get(3)?,
                name: row.get(4)?,
                progress: row.get(5)?,
                project: if let Ok(uuid) = Uuid::from_str(row.get::<usize, String>(6)?.as_str()) {
                    Some(uuid)
                } else {
                    None
                },

                status: row.get::<usize, String>(7)?.as_str().into(),
                date_info: row.get::<usize, String>(8)?.as_str().into(),

                history: try_get_history(row),
                is_history: row.get(10)?,
            })
        })?;

        for f in iter {
            res.push(f?);
        }
    }

    Ok(res)
}

pub fn match_sub(cmd: &str) -> &str {
    match cmd {
        "d" | "drop" | "delete" => "delete",
        "l" | "ls" => "ls",

        _ => {
            eprint_help();
        }
    }
}

pub fn eprint_help() -> ! {
    eprintln!(r#"HELP"#);
    std::process::exit(0);
}

pub fn safe_exit_with(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(0);
}

pub fn safe_exit() -> ! {
    std::process::exit(0);
}

pub fn try_get_history(row: &rusqlite::Row) -> Option<History> {
    if let Ok(history) = row.get::<usize, String>(9) {
        Some(serde_json::from_str(&history).expect(""))
    } else {
        None
    }
}

pub fn get_last_id(conn: &Connection) -> usize {
    // default last_id is 0
    let mut stmt = conn
        .prepare(
            "SELECT
            last_id
            FROM Self
            ",
        )
        .expect("");

    let mut rows = stmt.query([]).expect("");
    let mut res = 0;

    if let Some(row) = rows.next().expect("") {
        res = row.get(0).expect("");
    }

    println!("res:{}", res);
    res
}
