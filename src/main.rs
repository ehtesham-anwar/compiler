mod utils;
use std::io::Write;
use std::net::TcpStream;
use std::{collections::HashMap, net::TcpListener};
use utils::HttpRequest;
use utils::load_domains;
use utils::parse_request;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut domain_map: HashMap<String, String> = HashMap::new();
    load_domains("config/domain.yml", &mut domain_map);

    for stream in tcp_listener.incoming() {
        let mut stream: TcpStream = stream.unwrap();
        let peer = stream.peer_addr().unwrap(); // client IP + port

        let req: HttpRequest = parse_request(&stream, peer);

        println!("Request: {} {} from {}", req.method, req.path, req.host);

        let host_path = domain_map.get(&req.host);

        if host_path.is_none() {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Host Header";
            stream.write_all(response.as_bytes()).unwrap();
            continue;
        }

        // run php script in that directory
        // e,g /var/www/html
        // look for index.php or index.html
        // if index.php run php-cgi -f /var/www/html/index.php
        // if index.html serve that file
        // if neither serve 404

        let root_path = host_path.unwrap();
        let index_php = format!("{}/index.php", root_path);
        let index_html = format!("{}/index.html", root_path);
        let response = match std::path::Path::new(&index_php).exists() {
            true => {
                // run php-cgi
                let output = std::process::Command::new("php-cgi")
                    .arg("-f")
                    .arg(&index_php)
                    .output()
                    .expect("failed to execute process");
                let body = String::from_utf8_lossy(&output.stdout);
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                )
            }
            false => match std::path::Path::new(&index_html).exists() {
                true => {
                    let body = std::fs::read_to_string(&index_html).unwrap();
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    )
                }
                false => "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string(),
            },
        };
        stream.write_all(response.as_bytes()).unwrap();
    }
}
