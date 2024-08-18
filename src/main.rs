//      modules
//      =======
mod environment;
mod errors;
mod event_loop;
mod log;
mod request;
mod response;

//      imports
//      =======
// crate
use environment::Environment;
use event_loop::start_listener;
use log::{LogLevel, Logger};
// std
use std::{
    env,
    net::{SocketAddr, TcpListener},
    process::exit,
    sync::Arc,
};

//      functions
//      =========
fn main() {
    let mut args = env::args();
    let _ = args.next();
    let default = Environment::default();
    let environment = match Environment::from_args(default, args) {
        Ok(environment) => Arc::new(environment),
        Err(err) => {
            eprintln!("Error: {}", err);
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
