use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::net::TcpListener;
use::std::{io::Write,net::TcpStream}; 

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

fn handle_requset(mut stream :TcpStream){
   
   let reader = BufReader::new(&stream);
   let request = reader.lines().next().unwrap().unwrap();

   let status_line = match &request[..]{
     "GET / HTTP/1.1" => "HTTP/1.1 200 OK",
      _ => "HTTP/1.1 404 Not Found",
   };

   let response = format!("{status_line}\r\n\r\n");

   match stream.write_all(response.as_bytes()) {
    Ok(_)=>(),
    Err(e)=>println!("could not send respond:{:?}",e)
       
   }
}
