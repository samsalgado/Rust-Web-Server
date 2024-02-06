use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread;
use Rust_Web_Server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.expect("Failed to accept connection");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Buffer length 1024 bytes
    stream.read(&mut buffer).expect("Failed to read stream");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            let error_page = format!("{} Not Found", filename);
            return_error_response(&mut stream, "404 Not Found", &error_page);
            return;
        }
    };

    let response = format!(
        "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).expect("Failed to write response");
    stream.flush().expect("Failed to flush stream");
}

fn return_error_response(stream: &mut TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).expect("Failed to write response");
    stream.flush().expect("Failed to flush stream");
}
