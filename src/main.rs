use ::std::{io::Write, net::TcpStream};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader, Read},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("accepted new connection");
                thread::spawn(|| {
                    handle_requset(stream);

                    println!("stream_response: response send");
                });
            }
            Err(e) => {
                println!("stream_error error: {}", e);
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
    // let argv = env::args().collect::<Vec<String>>();

    // // let dir =argv[2].clone();
    // println!("{:?}", argv);

    reader.read_line(&mut request_line).unwrap();

    let request_line = request_line.trim_end();
    println!("request_line : {}", request_line);
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

    println!("request_header: {:?}", headers);

    let status_line;

    if parts.len() > 2 {
        let content = parts[1];
        let method = parts[0];

        if content.starts_with("/echo") {
            let main_content = content.strip_prefix("/echo/").unwrap();
            println!("main_content :{}", main_content);

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

            if method == "POST" {
                let content_length = headers.get("Content-Length").unwrap();
               let content_length = content_length.parse::<usize>().unwrap();
                println!("Content_length :{}", content_length);

                    let mut body_buff = vec![0u8; content_length];
                    reader.read_exact(&mut body_buff).unwrap();

                    let request_body =String::from_utf8_lossy(&body_buff).to_string();



                   println!("request_body : {} ",request_body);

                println!(
                    "from post: method ,request_body  {},{} ,{} ",
                    method, request_body, path
                );

                let off_path  = "/tmp/data/codecrafters.io/http-server-tester/";

                let main_path = format!("{off_path}{path}");
                match fs::write(main_path, request_body.to_string()) {
                    Ok(_) => {
                        status_line = format!("HTTP/1.1 201 Created\r\n\r\n");
                        println!(" from_write :{}", status_line)
                    }
                    Err(_) => status_line = format!("HTTP/1.1 404 Not Creates\r\n\r\n"),
                };
            } else {
                let path = format!("/tmp/data/codecrafters.io/http-server-tester/{path}");
                let main_content = fs::read_to_string(path);
                match main_content {
                    Ok(file) => {
                        let length = file.as_bytes().len();
                        status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{file}")
                    }
                    Err(e) => status_line = format!("HTTP/1.1 404 Not Found\r\n\r\n"),
                }
            }
        } else if content.len() == 1 {
            println!("content from length 1 ");
            status_line = format!("HTTP/1.1 200 OK\r\n\r\n");
        } else {
            status_line = format!("HTTP/1.1 404 Not Found\r\n\r\n");
        }

        match stream.write_all(status_line.as_bytes()) {
            Ok(res) => {
                println!("success_from stream_write:{:?}", res)
            }
            Err(e) => println!("error_from_strean_write could not send respond:{:?}", e),
        }
    }
}
