use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    /* parse arguments */
    let uri = match std::env::args().nth(1) {
        Some(uri) => uri,
        None => {
            println!("usage: blogstage <URI> <PATH>");
            return;
        }
    };

    let path = match std::env::args().nth(2) {
        Some(path) => path,
        None => {
            println!("usage: blogstage <URI> <PATH>");
            return;
        }
    };

    /* load files */
    let raw_entries = match fs::read_dir(path.clone()) {
        Ok(entries) => entries,
        Err(e) => {
            println!("error reading files from {}: {}", path, e);
            return;
        }
    };

    let mut files = HashMap::new();

    for entry in raw_entries {
        if !entry.as_ref().unwrap().path().is_file() {
            continue;
        }

        files.insert(
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .clone(),
            entry.as_ref().unwrap().path().clone(),
        );
    }

    /* start server */
    let listener = match TcpListener::bind(uri.clone()) {
        Ok(l) => l,
        Err(e) => {
            println!("error on binding to {}: {}", uri, e);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(s) => on_request(s, files.clone()),
            Err(e) => {
                println!("error accepting connection: {}", e);
                continue;
            }
        }
    }
}

fn on_request(mut stream: TcpStream, files: HashMap<String, PathBuf>) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut target: String = request[0].split(' ').collect::<Vec<&str>>()[1][1..].to_string();

    if target == "" {
        target = "index.html".into()
    }

    let response = match files.get(&target) {
        Some(path) => {
            let body = fs::read_to_string(path).unwrap();
            let length = body.len();

            println!("200 {}", target);
            format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{body}")
        }
        None => {
            println!("404 {}", target);
            "HTTP/1.1 404 NOT FOUND\r\n\r\n".into()
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}
