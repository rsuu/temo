use rusqlite::Connection;
use temo::{
    cli,
    data::{config, db},
};

#[tokio::main]
async fn main() {
    let config: config::Config = config::Config::read("./config.toml");
    // println!("{:#?}", config);

    let conn = Connection::open(config.misc.db_path.as_str()).expect("");
    let _db = db::Db { conn };

    // let mut task = task::Task::new(1, "test1");
    // task.set_progress("awdawd");

    //   println!("{:#?}", task);

    cli::parse::parse_args(&config).await.expect("");
}
