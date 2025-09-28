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

use std::net::SocketAddr;

pub struct HttpRequest {
    pub host: String,
    pub method: String,
    pub path: String,
    pub body: String,
    pub user_agent: String
}

use std::net::TcpStream;
use std::str;

pub fn parse_request(stream: &TcpStream, peer: SocketAddr) -> HttpRequest {
    let mut buffer = [0; 1024];
    stream.peek(&mut buffer).unwrap(); // peek without consuming
    let request_str = String::from_utf8_lossy(&buffer);

    let mut lines = request_str.lines();

    // Host
    let mut host = String::new();
    for line in lines.clone() {
        if line.to_lowercase().starts_with("host:") {
            host = line["Host:".len()..].trim().to_string();
            break;
        }
    }

    if host.starts_with("localhost") || host.starts_with("127.0.0.1") {
        host = "localhost".to_string();
    }

    // First line: METHOD PATH PROTOCOL
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();

    // Headers
    let mut user_agent = String::new();
    for line in lines.clone() {
        if line.to_lowercase().starts_with("user-agent:") {
            user_agent = line["User-Agent:".len()..].trim().to_string();
            break;
        }
    }

    // Body (everything after \r\n\r\n)
    let body = if let Some(pos) = request_str.find("\r\n\r\n") {
        request_str[(pos + 4)..].to_string()
    } else {
        String::new()
    };

    HttpRequest {
        host,
        method,
        path,
        body,
        user_agent
    }
}
