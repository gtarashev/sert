mod environment;
mod errors;
mod log;
mod request;
mod response;

use environment::Environment;
use log::{LogLevel, Logger};
use response::handle_client;
use std::{
    net::{SocketAddr, TcpListener},
    process::exit,
    sync::Arc,
    thread,
};

fn main() {
    let environment = match Environment::from_args() {
        Ok(environment) => Arc::new(environment),
        Err(err) => {
            eprintln!("Error while parsing arguments: {}", err);
            exit(1);
        }
    };

    let logger = Arc::new(Logger::new(environment.time.to_string(), environment.color));
    logger.log(LogLevel::Info, "Started up logger.");
    logger.log(
        LogLevel::Info,
        format!("Using configuration:\n{}", environment),
    );

    let addr = SocketAddr::new(environment.address.into(), environment.port);
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
                let env = Arc::clone(&environment);
                thread::spawn(move || {
                    handle_client(&env, &logger, stream);
                });
            }
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}
