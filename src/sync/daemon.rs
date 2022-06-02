use std::io::Error;

use signal_hook::{consts::signal::*, iterator::Signals};

use std::{
    process::{exit, Command},
    thread,
    thread::sleep,
    time::Duration,
};

use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};

pub fn run_daemon() {
    unsafe {
        match fork().expect("Failed to fork process") {
            ForkResult::Parent { child } => {
                println!("Try to kill me to check if the target process will be killed");

                // Do not forget to wait for the fork in order to prevent it from becoming a zombie!!!
                waitpid(Some(child), None).unwrap();

                // You have 120 seconds to kill the process :)
                sleep(Duration::from_secs(2));
            }

            ForkResult::Child => {
                // replace with your executable
                Command::new("/tmp/cargo/target/release/temo")
                    .arg("--sync")
                    .arg("s")
                    .spawn()
                    .expect("failed to spawn the target process");
                exit(0);
            }
        }
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
