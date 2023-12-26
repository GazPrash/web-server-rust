use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct HTTPResponse {
    status_line: String,
    response_content: String,
    response_len: usize,
}

fn handle_valid(path: &str) -> HTTPResponse {
    let read_filename = match path {
        "/home" => "data/response.html",
        "/about" => "data/about.html",
        _ => "data/notfound.html",
    };
    let status_line = "HTTP/1.1 200 OK".to_string();
    let response_content = fs::read_to_string(format!("data/{read_filename}")).unwrap();
    let response_len = response_content.len();
    HTTPResponse {
        status_line,
        response_content,
        response_len,
    }
}

fn handle_invalid() -> HTTPResponse {
    let status_line = "HTTP/1.1 404 NOT FOUND".to_string();
    let response_content = fs::read_to_string("data/notfound.html").unwrap();
    let response_len = response_content.len();
    HTTPResponse {
        status_line,
        response_content,
        response_len,
    }
}
fn handle_connection_tcp(mut stream: TcpStream) {
    println!("Connection Estb.");
    let buff_reader = BufReader::new(&mut stream);
    let request = buff_reader.lines().next().unwrap().unwrap();
    println!("Request : {}", request);
    let httpresponse = match request.as_str() {
        "GET / HTTP/1.1" => handle_valid("/home"),
        "GET /about HTTP/1.1" => handle_valid("/about"),
        _ => handle_invalid(),
    };
    let status_line = httpresponse.status_line;
    let resp_content = httpresponse.response_content;
    let resp_len = httpresponse.response_len;

    let server_response =
        format!("{status_line}\r\nContent-Length: {resp_len}\r\n\r\n{resp_content}");
    stream.write_all(server_response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection_tcp(stream);
    }
}
