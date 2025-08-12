use ::std::{io::Write, net::TcpStream};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("accepted new connection");
                thread::spawn(|| handle_requset(stream));
                println!("response send")
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// fn handle_stream( mut stream :TcpStream){
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write_all(response.as_bytes()).unwrap();

// }

fn handle_requset(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let request_line = request_line.trim_end();

    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(":") {
            headers.insert(name.trim().to_string(), value.trim().to_string());
        }
    }

    println!("{:?}", headers);

    let status_line;

    if parts.len() > 2 {
        let content = parts[1];

        if content.starts_with("/echo") {
            let main_content = content.strip_prefix("/echo/").unwrap();
            println!("{}", main_content);

            let length = main_content.len();
            status_line = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{length} \r\n\r\n{main_content}"
            );
        } else if content.starts_with("/user-agent") {
            let main_content = headers.get("User-Agent").unwrap();
            let length = main_content.len();
            status_line = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{length} \r\n\r\n{main_content}"
            );
        } else if content.starts_with("/files") {
            let path = content.strip_prefix("/files/").unwrap();
            let path = format!("/tmp/{path}");
            let main_content = fs::read_to_string(path);
            match main_content {
                Ok(file) => {
                    let length = file.as_bytes().len();
                    status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{file}")
                }
                Err(e) => status_line = format!("HTTP/1.1 404 Not Found\r\n\r\n"),
            }
        } else if content.len() == 1 {
            status_line = format!("HTTP/1.1 200 OK\r\n\r\n");
        } else {
            status_line = format!("HTTP/1.1 404 Not Found\r\n\r\n");
        }
        match stream.write_all(status_line.as_bytes()) {
            Ok(_) => (),
            Err(e) => println!("could not send respond:{:?}", e),
        }
    }
}
