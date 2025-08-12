use ::std::{io::Write, net::TcpStream};
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("accepted new connection");
                handle_requset(stream);
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
     let status_line : String;
    let reader = BufReader::new(&stream);

    // println!("{:?}",reader);
    let request = reader.lines().next().unwrap().unwrap();
    let parts: Vec<&str> = request.split_whitespace().collect();
    println!("{:?}", parts);

    //    let status_line;

    if parts.len() > 2 {
        let content = parts[1];
       
   

        if content.starts_with("/echo") {
            let main_content = content.strip_prefix("/echo/").unwrap();
            println!("{}",main_content);
            
           let length = main_content.len();
             status_line = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{length} \r\n\r\n{main_content}"
            );
        }else if content.len()==1 {
             status_line = format!("HTTP/1.1 200 OK\r\n\r\n");

        }else {
                     status_line = format!("HTTP/1.1 404 Not Found\r\n\r\n");

        }
         match stream.write_all(status_line.as_bytes()) {
        Ok(_)=>(),
        Err(e)=>println!("could not send respond:{:?}",e)

       }
    }

   

    

      
}
