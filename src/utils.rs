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
    pub user_agent: String,
    pub client_ip: String,
}

use std::net::TcpStream as StdTcpStream;
use std::str;

pub fn parse_request(stream: &StdTcpStream, peer: SocketAddr) -> HttpRequest {
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
        user_agent,
        client_ip: peer.ip().to_string(),
    }
}

use std::process::Command;

pub fn run_php_script(request: &HttpRequest, script_path: &str) -> String {
    // Simple PHP execution using php-cgi with proper CGI environment
    let output = Command::new("php-cgi")
        .arg("-f")
        .arg(script_path)
        .env("REDIRECT_STATUS", "200") // Required for PHP CGI security
        .env("REQUEST_METHOD", &request.method)
        .env("REQUEST_URI", &request.path)
        .env("SCRIPT_FILENAME", script_path)
        .env("SCRIPT_NAME", &request.path)
        .env("PATH_INFO", "")
        .env("QUERY_STRING", extract_query_string(&request.path))
        .env("CONTENT_TYPE", "application/x-www-form-urlencoded")
        .env("CONTENT_LENGTH", request.body.len().to_string())
        .env("HTTP_HOST", &request.host)
        .env("HTTP_USER_AGENT", &request.user_agent)
        .env("REMOTE_ADDR", &request.client_ip)
        .env("SERVER_NAME", &request.host)
        .env("SERVER_PORT", "8080")
        .env("SERVER_PROTOCOL", "HTTP/1.1")
        .env("GATEWAY_INTERFACE", "CGI/1.1")
        .stdin(std::process::Stdio::piped())
        .output();
    match output {
        Ok(result) => {
            if result.status.success() {
                let php_output = String::from_utf8_lossy(&result.stdout);
                // Check if PHP output looks like it has headers already
                if php_output.contains("Content-Type:") && php_output.contains("\r\n\r\n") {
                    // PHP provided complete CGI response, just add HTTP status
                    let response = format!("HTTP/1.1 200 OK\r\n{}", php_output);
                    response
                } else {
                    // PHP output is just content, add our own headers
                    let clean_output = php_output.trim();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        clean_output.len(),
                        clean_output
                    );
                    response
                }
            } else {
                let error = String::from_utf8_lossy(&result.stderr);
                let stdout = String::from_utf8_lossy(&result.stdout);
                println!("❌ PHP Error - STDERR: {}, STDOUT: {}", error, stdout);
                format!(
                    "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\n\r\n<h1>PHP Error</h1><pre>STDERR: {}\nSTDOUT: {}</pre>",
                    error, stdout
                )
            }
        }
        Err(e) => {
            format!(
                "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to execute PHP: {}",
                e
            )
        }
    }
}

fn extract_query_string(path: &str) -> String {
    if let Some(pos) = path.find('?') {
        path[(pos + 1)..].to_string()
    } else {
        String::new()
    }
}
