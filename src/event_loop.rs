use crate::response::handle_client;
use crate::log::{LogLevel, Logger};
use crate::environment::Environment;

use termios::{tcsetattr, Termios};
use termios::{ICANON, IEXTEN, ISIG, TCSANOW};

use std::{
    io::{stdin, Read},
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    net::TcpListener,
    thread,
    time::Duration,
};

const STDIN_FILENO: i32 = 0;
const CTRL_C: [u8; 3] = [3, 0, 0];

fn event_loop(listener: Arc<TcpListener>, logger: Arc<Logger>, environment: Arc<Environment>, stop: Arc<AtomicBool>) {
    for stream in listener.incoming() {
        // interrupt for stopping the thread. since `listener.incoming()` is blocking
        // it will only run when a request comes in. this means that there might be
        // a need to force stop the thread after some times since there is no
        // guarantee otherwise
        if stop.load(Ordering::SeqCst) {
            logger.log(LogLevel::Info, "Event loop thread received stop signal, exiting.");
            break;
        }

        logger.log(LogLevel::Info, "Serving client... ");
        match stream {
            Ok(stream) => {
                let logger = Arc::clone(&logger);
                let env = Arc::clone(&environment);
                thread::spawn(
                    move || {
                    handle_client(&env, &logger, stream);
                });
            }
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}

fn init_term() -> Termios {
    let termios = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ISIG | IEXTEN);
    tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();
    return termios;
}

fn reset_term(termios: &Termios) {
    tcsetattr(STDIN_FILENO, TCSANOW, termios).unwrap();
}

pub fn start_listener(
    listener: Arc<TcpListener>,
    env: Arc<Environment>,
    logger: Arc<Logger>,
) {
    let stop = Arc::new(AtomicBool::new(false));
    let loop_handle = thread::spawn({
        let stop = Arc::clone(&stop);
        let listener = Arc::clone(&listener);
        let env = Arc::clone(&env);
        let logger = Arc::clone(&logger);
    move || event_loop(listener, logger, env, stop)});

    // enter raw mode
    logger.log(LogLevel::Info, "Initialising input handler");
    let termios = init_term();
    let mut buffer = [0u8; 3];
    let mut stdin = stdin();
    loop {
        let _ = stdin.read(&mut buffer);
        if buffer == CTRL_C {
            stop.store(true, Ordering::Relaxed);
            break;
        }
    }
    logger.log(LogLevel::Warn, "Received terminating signal.");
    if !loop_handle.is_finished() {
        logger.log(LogLevel::Warn, "Request handler has not finished. Waiting 5 more seconds.");
        thread::sleep(Duration::from_secs(5));
    }

    if !loop_handle.is_finished() {
        logger.log(LogLevel::Warn, "Request handler has not finished. Will terminate.");
    }
    reset_term(&termios);
    logger.log(LogLevel::Null, "Terminating");
}
