use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_request(stream);
        });
    }
}

fn handle_request(mut stream: TcpStream) {
    let response = "HTTP/1.1 418 I'm a teapot\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
