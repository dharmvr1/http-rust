#[allow(unused_imports)]
use std::net::TcpListener;
use::std::{io::Write,net::TcpStream}; 

fn main() {
  

  
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("accepted new connection");
                handle_stream(stream);
                println!("response send")

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    
}


fn handle_stream( mut stream :TcpStream){
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    
}
