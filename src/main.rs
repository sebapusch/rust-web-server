use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug)]
struct HttpRequest {
    method: String,
    uri: String,
    protocol: String,
}

fn main() {
    start_listening();
}

fn start_listening() {

    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = web_server::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
           handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {

    let request = parse_request(&stream);

    if request.uri == "/sleep" {
        thread::sleep(Duration::from_secs(5));
    }

    println!("{:?}", request);

    let status = "HTTP/1.1 200 OK";
    let body = fs::read_to_string("hello.html").unwrap();
    let len = body.len();

    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();

}

fn parse_request(stream: &TcpStream) -> HttpRequest {
    let buf_reader = BufReader::new( stream);
    let http_request : Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0].split(" ").collect::<Vec<&str>>();

    return HttpRequest {
        method: request_line[0].parse().unwrap(),
        uri: request_line[1].parse().unwrap(),
        protocol: request_line[2].parse().unwrap(),
    };
}
