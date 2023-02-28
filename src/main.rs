use std::collections::HashMap;
use std::fs;
use std::net::TcpListener;
use std::thread;

use blogstage::on_request;

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

    // react to Ctrl+C
    ctrlc::set_handler(move || {
            std::process::exit(0);
    }).unwrap();

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

