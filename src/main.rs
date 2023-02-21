use std::fs;
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main(){
    /* parse arguments */
    let uri = match std::env::args().nth(1){
        Some(uri) => uri,
        None => {
            println!("usage: blogstage <URI> <PATH>");
            return
        }
    };

    let path = match std::env::args().nth(2) {
        Some(path) => path,
        None => {
            println!("usage: blogstage <URI> <PATH>");
            return
        }
    };

    /* load files */
    let raw_entries = match fs::read_dir(path.clone()) {
        Ok(entries) => entries,
        Err(e) => {
            println!("error reading files from {}: {}", path, e);
            return
        }
    };

    let entries = raw_entries.filter(|entry| entry.as_ref().unwrap().path().is_file())
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    /* start server */
    let listener = match TcpListener::bind(uri.clone()) {
        Ok(l) => l,
        Err(e) => {
            println!("error on binding to {}: {}", uri, e);
            return
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(s) => on_request(s, entries.clone(), path.clone()),
            Err(e) => {
                println!("error accepting connection: {}", e);
                continue
            }
        }
    }
}

fn on_request(mut stream: TcpStream, files: Vec<std::ffi::OsString>, path: String) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:?}", request);
    println!("{}: {:?}", path, files);

    let response = "HTTP/1.1 200 OK\r\n\r\nblogstage\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
