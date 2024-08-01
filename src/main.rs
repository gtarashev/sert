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
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn listener_loop(
    listener: TcpListener,
    environment: Arc<Environment>,
    logger: Arc<Logger>,
    stop: Arc<AtomicBool>,
) {
    for stream in listener.incoming() {
        // interrupt for stopping the thread. since `listener.incoming()` is blocking
        // it will only run when a request comes in. this means that there might be
        // a need to force stop the thread after some times since there is no
        // guarantee otherwise
        if !stop.load(Ordering::SeqCst) {
            break;
        }

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

fn main() {
    let environment = match Environment::from_args() {
        Ok(environment) => Arc::new(environment),
        Err(err) => {
            eprintln!("Error while parsing arguments: {}", err);
            exit(1);
        }
    };
    let stop = Arc::new(AtomicBool::new(false));

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

    let _listener_handle = thread::spawn({
        let stop = stop.clone();
        let env = environment.clone();
        let logger = logger.clone();
        move || {
            listener_loop(listener, env, logger, stop);
        }
    });
}
