use mime_guess;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

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
            fs::read(entry.as_ref().unwrap().path().clone()).unwrap(),
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
            Ok(s) => {
                let f = files.clone();
                thread::spawn(move || on_request(s, f));
            }
            Err(e) => {
                println!("error accepting connection: {}", e);
                continue;
            }
        }
    }
}

fn on_request(mut stream: impl Read + Write, files: HashMap<String, Vec<u8>>) {
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

    let mime = mime_guess::from_path(target.clone()).first().unwrap();

    match files.get(&target) {
        Some(body) => {
            let length = body.len();

            println!("200 {}", target);
            stream.write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: {mime}\r\n\r\n")
                .as_bytes()
                ).unwrap();
            stream.write_all(&body).unwrap();
        }
        None => {
            println!("404 {}", target);
            stream
                .write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())
                .unwrap();
        }
    };
}
