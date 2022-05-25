use std::path::Path;

fn load_file(path: &str) {
    let path = Path::new(path);

    if path.exists() {
    } else {
        eprintln!("CRITICAL: Could not open config file: {:?}", path);
        std::process::exit(-1)
    }
}
