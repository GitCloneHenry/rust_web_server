use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let mut lines = request.lines();

    // Request line: GET /path HTTP/1.1
    let request_line = lines.nth(0).unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let path = parts.nth(1).unwrap_or("/");

    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    let accept_line = lines.nth(3).unwrap_or("");
    let mut parts = accept_line.split_whitespace();
    let accept_details = parts.nth(1).unwrap_or("");
    let mut accept_parts = accept_details.split(";");
    let mut accept_types = accept_parts.next().unwrap_or("*/*").split(",");
    let accept_type = accept_types.next().unwrap_or("");

    // Build file path
    let filepath = if path == "/" {
        "./src/index.html".to_string()
    } else {
        format!("./src{}", path)
    };

    // Try to read file
    let (status, body) = match fs::read_to_string(&filepath) {
        Ok(contents) => ("200 OK", contents),
        Err(_) => ("404 NOT FOUND", "<h1>404 - Not Found</h1>".to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
        status,
        accept_type,
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
    Ok(())
}
