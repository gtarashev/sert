//      imports
//      =======
// crate
use crate::{
    environment::Environment,
    log::{LogLevel, Logger},
    response::handle_client,
};
// std
use std::{
    io::{stdin, Read},
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
// termios
use termios::{tcsetattr, Termios, ECHO, ICANON, IEXTEN, ISIG, TCSANOW};

//      constants
//      =========
const STDIN_FILENO: i32 = 0;
const CTRL_C: [u8; 3] = [3, 0, 0];

//      functions
//      =========
fn event_loop(
    listener: Arc<TcpListener>,
    logger: Arc<Logger>,
    environment: Arc<Environment>,
    stop: Arc<AtomicBool>,
) {
    for stream in listener.incoming() {
        // interrupt for stopping the thread. since `listener.incoming()` is blocking
        // it will only run when a request comes in. this means that there might be
        // a need to force stop the thread after some times since there is no
        // guarantee otherwise
        if stop.load(Ordering::SeqCst) {
            logger.log(
                LogLevel::Info,
                "Event loop thread received stop signal, exiting.",
            );
            break;
        }

        logger.log(LogLevel::Info, "Serving client... ");
        match stream {
            Ok(stream) => {
                thread::spawn({
                    let logger = Arc::clone(&logger);
                    let env = Arc::clone(&environment);
                    move || {
                        handle_client(&env, &logger, stream);
                    }
                });
            }
            Err(err) => {
                logger.log(LogLevel::Error, err);
            }
        }
    }
}

// --------
fn init_term() -> Termios {
    let termios = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();
    return termios;
}

// --------
fn reset_term(termios: &Termios) {
    tcsetattr(STDIN_FILENO, TCSANOW, termios).unwrap();
}

// --------
pub fn start_listener(listener: Arc<TcpListener>, env: Arc<Environment>, logger: Arc<Logger>) {
    let stop = Arc::new(AtomicBool::new(false));
    let loop_handle = thread::spawn({
        let stop = Arc::clone(&stop);
        let listener = Arc::clone(&listener);
        let env = Arc::clone(&env);
        let logger = Arc::clone(&logger);
        move || event_loop(listener, logger, env, stop)
    });

    // enter raw mode
    logger.log(LogLevel::Info, "Initialising input handler");
    let termios = init_term();
    let mut buffer = [0u8; 3];
    let mut stdin = stdin();
    loop {
        // listen on stdin, match on certain sequences
        let _ = stdin.read(&mut buffer);
        if buffer == CTRL_C {
            stop.store(true, Ordering::Relaxed);
            break;
        }
    }
    logger.log(LogLevel::Warn, "Received terminating signal.");
    if !loop_handle.is_finished() {
        logger.log(
            LogLevel::Warn,
            format!(
                "Request handler has not finished. Waiting {} more seconds.",
                env.timeout
            ),
        );
        thread::sleep(Duration::from_millis(env.timeout));
    }

    if !loop_handle.is_finished() {
        logger.log(
            LogLevel::Warn,
            "Request handler has not finished. Will terminate.",
        );
    }
    reset_term(&termios);
    logger.log(LogLevel::Null, "Terminating");
}
