pub mod cli {
    pub mod filter;
    pub mod parse;
    pub mod args {
        pub mod config;
        pub mod create;
        pub mod sync;
    }

    pub mod subs {
        pub mod info;
        pub mod pomodoro;
        pub mod project;
        pub mod work;
    }
}

pub mod utils {}

pub mod data {
    pub mod config;
    pub mod db;
    pub mod json;
    pub mod task;
}

pub mod sync {
    pub mod client;
    pub mod daemon;
    pub mod server;
}
