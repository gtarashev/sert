mod environment;
mod errors;
mod log;
mod request;
mod response;
mod event_loop;

use environment::Environment;
use log::{LogLevel, Logger};
use event_loop::start_listener;
use std::{
    net::{SocketAddr, TcpListener},
    process::exit,
    sync::Arc,
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
        Ok(listener) => Arc::new(listener),
        Err(err) => {
            logger.log(LogLevel::Error, err);
            exit(1);
        }
    };

    start_listener(listener, environment, logger);
}
