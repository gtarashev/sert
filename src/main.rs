mod errors;
mod log;
mod request;

use log::{LogLevel, Logger};
use request::{HttpMethod, Request};
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process::exit,
    sync::Arc,
    thread,
};

fn handle_client(logger: &Logger, stream: TcpStream) {
    let mut request = match Request::try_from(stream) {
        Ok(request) => request,
        Err(_) => {
            logger.log(LogLevel::Error, "Error parsing TCP stream.");
            return;
        }
    };

    let status = match request.method {
        HttpMethod::GET => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 NOT FOUNT",
    };

    let file = match &request.content[..] {
        "/" | "/test.html" | "test.html" => "html/test.html",
        _ => "html/not_found.html",
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, contents
    );

    match request.writer.write(response.as_bytes()) {
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
