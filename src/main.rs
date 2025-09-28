use std::net::TcpListener;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        &args[1]
    } else {
        "7878"
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connection established!");
    }

    Ok(())
}
