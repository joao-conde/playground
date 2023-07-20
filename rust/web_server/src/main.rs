use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let tpool = ThreadPool::new(5).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        tpool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down web server");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";
    let sleep_request = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "res/index.html")
    } else if buffer.starts_with(sleep_request) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "res/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "res/404.html")
    };

    let response_data = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status, response_data);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
