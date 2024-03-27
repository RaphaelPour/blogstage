use std::collections::HashMap;
use std::io::{prelude::*, BufReader, Read, Write};

// We need to put everything out of main.rs what should be tested via integration tests:
// https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates

pub fn on_request(mut stream: impl Read + Write, files: HashMap<String, Vec<u8>>) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut target: String = request[0].split(' ').collect::<Vec<&str>>()[1][1..].to_string();

    if target.is_empty() {
        target = "index.html".into()
    } else if target.contains("?") {
        // chop query
        target = target.split('?').collect::<Vec<&str>>()[0].to_string();
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
            stream.write_all(body).unwrap();
        }
        None => {
            println!("404 {}", target);
            stream
                .write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())
                .unwrap();
        }
    };
}
