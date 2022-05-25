use std::{
    io::Error,
};

use fork::{daemon, Fork};
use signal_hook::{
    consts::signal::*,
    iterator::Signals,
};

use std::process::Command;
use std::thread;


pub fn run_daemon() {
    if let Ok(Fork::Child) = daemon(false, false) {
        Command::new("echo")
            .arg("1")
            .spawn()
            .expect("Child process failed to start.");
    }
}

pub fn get_signal() -> Result<(), Box<Error>> {
    let mut signals = Signals::new(&[SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    Ok(())
}

// REF
// https://stackoverflow.com/questions/26280859/how-to-catch-signals-in-rust
