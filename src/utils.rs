use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_domains(file_path: &str, domain_map: &mut HashMap<String, String>) {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("Unable to open domains file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read domains file");

    for line in contents.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let domain: String = parts[0].trim().to_string();
            let root_path: String = parts[1].trim().to_string();
            domain_map.insert(domain, root_path);
        } else {
            // throw error gracefully exiting the program
            let domain_syntax_error = "Domain syntax error : localhost: /path/to/root";
            eprintln!("Invalid line in domains file: {}", line);
            eprintln!("{}", domain_syntax_error);
            std::process::exit(1);
        }
    }
}
