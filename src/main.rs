mod log;

use log::{LogLevel, Logger};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
    sync::Arc,
    thread,
};

fn handle_client(logger: &Logger, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    logger.log(LogLevel::Info, format!("Request: {http_request:#?}"));
    if http_request.len() == 0 {
        return;
    }

    let (status, file) = match &http_request[0][..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/test.html"),
        _ => ("HTTP/1.1 404 NOT FOUNT", "html/not_found.html"),
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, contents
    );

    match stream.write_all(response.as_bytes()) {
        Ok(_) => logger.log(LogLevel::Info, "Client served successfully"),
        Err(_) => logger.log(LogLevel::Error, "Error sending data."),
    }
}

fn main() {
    let logger = Arc::new(Logger::new(true, true));
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
                let logger = Arc::clone(&logger);
                thread::spawn(move || {
                    handle_client(&logger, stream);
                });
            }
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}
