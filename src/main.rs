use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let mut lines = request.lines();

    // Request line: GET /path HTTP/1.1
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    println!("{} {}", method, path);

    // Build file path
    let filepath = if path == "/" {
        "./src/index.html".to_string()
    } else {
        format!("./src{}", path)
    };

    // Guess content type by extension
    let content_type = if filepath.ends_with(".html") {
        "text/html"
    } else if filepath.ends_with(".css") {
        "text/css"
    } else if filepath.ends_with(".js") {
        "application/javascript"
    } else {
        "text/plain"
    };

    // Try to read file
    let (status, body) = match fs::read_to_string(&filepath) {
        Ok(contents) => ("200 OK", contents),
        Err(_) => ("404 NOT FOUND", "<h1>404 - Not Found</h1>".to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
        status,
        content_type,
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
    Ok(())
}
