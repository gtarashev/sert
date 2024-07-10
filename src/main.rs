mod log;

use log::{Logger, LogLevel};
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::process::exit;

fn handle_client(logger: &Logger, mut stream: TcpStream) {
    match stream.write("Hello, World!".as_bytes()) {
        Ok (_) => (),
        Err(err) => {
            logger.log(LogLevel::Error, err);
        }
    }
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
            },
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}
