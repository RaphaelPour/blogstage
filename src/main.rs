use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main(){
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            println!("error on binding to 127.0.0.1:8080: {}", e);
            return
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(s) => on_request(s),
            Err(e) => {
                println!("error accepting connection: {}", e);
                continue
            }
        }
    }
}

fn on_request(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\nblogstage\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
