use std::fs::File;
use std::io::Read;
use std::path::Path;
fn main() {
    println!("Hello, world! This is web server");
    // Start the server here
    // 1. Load configuration from files
    let config = load_config("config.toml");
    println!("Loaded config: {}", config);
    // 2. Set up TCP listener
    // 3. Handle incoming connections
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream);
    // }
}

fn load_config(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read config file");
    contents
}
