mod errors;
mod log;
mod request;
mod response;
mod environment;

use log::{LogLevel, Logger};
use response::handle_client;
use environment::Environment;
use std::{net::TcpListener, process::exit, sync::Arc, thread};

fn main() {
    let environment = match Environment::from_args() {
        Ok(environment) => environment,
        Err(err) => {
            eprintln!("Error while parsing arguments: {}", err);
            exit(1);
        }
    };

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
