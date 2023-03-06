use blogstage;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::process::Command;

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

// https://doc.rust-lang.org/std/io/trait.Read.html
impl Read for MockTcpStream {
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Ok(size)
    }
}

// https://doc.rust-lang.org/std/io/trait.Write.html
impl Write for MockTcpStream {
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
        self.write_data.extend(buf.iter().cloned());
        Ok(buf.len())
    }

    fn flush(self: &mut Self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn uri_is_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blogstage")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("usage: blogstage <URI> <PATH>\n"));
    Ok(())
}

#[test]
fn path_is_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blogstage")?;

    cmd.arg("127.0.0.1:8080");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("usage: blogstage <URI> <PATH>\n"));
    Ok(())
}

#[test]
fn path_not_existing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blogstage")?;

    cmd.arg("127.0.0.1:8080");
    cmd.arg("/nope");
    cmd.assert()
        .success()
        .stdout("error reading files from /nope: No such file or directory (os error 2)\n");
    Ok(())
}

#[test]
fn bad_uri() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blogstage")?;

    cmd.arg("bad:uri");
    cmd.arg("./");
    cmd.assert()
        .success()
        .stdout("error on binding to bad:uri: invalid port value\n");
    Ok(())
}

#[test]
fn serve_index() {
    let input = b"GET /index.html HTTP/1.1\r\n\r\n";
    let mut contents = vec![0u8; 1024];

    contents[..input.len()].clone_from_slice(input);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    let expected_contents = "just works";
    let mut files = HashMap::new();
    files.insert(
        String::from("index.html"),
        expected_contents.as_bytes().to_vec(),
    );

    blogstage::on_request(&mut stream, files);

    let expected_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        expected_contents.len(),
        expected_contents
    );
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}

#[test]
fn serve_not_found() {
    let input = b"GET /test.html HTTP/1.1\r\n\r\n";
    let mut contents = vec![0u8; 1024];

    contents[..input.len()].clone_from_slice(input);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    let files = HashMap::new();
    blogstage::on_request(&mut stream, files);

    let expected_response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}
