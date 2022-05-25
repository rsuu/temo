
use serde_derive::{Deserialize, Serialize};
//use smartstring::alias::String as Sstr;
use std::fs;
use time::{format_description::well_known::Rfc3339};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Config {
    pub base: Base,
    pub misc: Misc,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Base {
    pub config_path: String,
    pub cache_path: String,

    pub author: String,
    pub config_create_at: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Misc {
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base: Base::default(),
            misc: Misc::default(),
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        Self {
            config_path: String::default(),
            cache_path: String::default(),

            author: String::default(),
            config_create_at: time::OffsetDateTime::now_utc().format(&Rfc3339).expect(""),
        }
    }
}

impl Default for Misc {
    fn default() -> Self {
        Self {
            db_path: "./temo.db".to_string(),
        }
    }
}

impl Config {
    pub fn read(path: &str) -> Self {
        // let toml = toml::to_string(&Config::default());
        // println!("{:#?}", toml);
        if let Ok(text) = fs::read_to_string(path) {
            toml::from_str(&text).expect("")
        } else {
            todo!()
        }
    }

    pub fn write(&self) -> String {
        toml::to_string(self).expect("")
    }

    pub fn edit(&mut self) {}
}
