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
    }

    let mime_guess = mime_guess::from_path(target.clone()).first();

    match files.get(&target) {
        Some(body) => {
            let length = body.len();
            // finalize the mime type
            let mime = mime_guess.unwrap_or(
                // Default mime is text extension, so at least the file content can be read.
                // Don't worry that it's being unwrapped unchecked, the value is ensured.
                // Who ensures the value? It's me! @Tch1b0!
                mime_guess::from_ext("txt".into()).first().unwrap()
            );

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
