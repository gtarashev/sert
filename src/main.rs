mod log;

use log::{LogLevel, Logger};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
};

fn handle_client(logger: &Logger, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    logger.log(LogLevel::Info, format!("Request: {http_request:#?}"));

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("html/test.html").unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let logger = Logger::new(true, true);
    logger.log(LogLevel::Info, "Starting up");

    let addr = "127.0.0.1:6969";
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => listener,
        Err(err) => {
            logger.log(LogLevel::Error, err);
            exit(1);
        }
    };

    for stream in listener.incoming() {
        logger.log(LogLevel::Info, "Serving client... ");
        match stream {
            Ok(stream) => {
                handle_client(&logger, stream);
            }
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}
