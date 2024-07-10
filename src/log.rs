use std::io::{self, Write};
use std::fmt;
use std::time;
use termion::color;
use chrono::offset::Utc;
use chrono::DateTime;

pub enum LogLevel {
    Null,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => write!(f, ""),
            Self::Info => write!(f, "INFO: "),
            Self::Warn => write!(f, "WARN: "),
            Self::Error => write!(f, "ERROR: "),
        }
    }
}

pub struct Logger {
    time: bool,
    color: bool,
}

// generic implementations for logger that dont require a logger object
impl Logger {
    pub fn new(time: bool, color: bool) -> Self {
        Self {
            time,
            color,
        }
    }
}

// functions that are called on a logger object
impl Logger {
    pub fn log<T: fmt::Display>(&self, log_level: LogLevel, msg: T) {
        let mut writer = Vec::new();
        // prepend the time if the option is set
        if self.time {
            let now = time::SystemTime::now();
            let datetime: DateTime<Utc> = now.into();
            let _ = write!(writer, "[{}] ", datetime.format("%d-%m-%Y %T"));
        }

        if self.color {
            match log_level {
                LogLevel::Null => (),
                LogLevel::Info => _ = write!(writer, "{}", color::Fg(color::Blue)),
                LogLevel::Warn => _ = write!(writer, "{}", color::Fg(color::Yellow)),
                LogLevel::Error => _ = write!(writer, "{}",color::Fg(color::Red)),
            };
        }

        let _ = write!(writer, "{}{}{}", log_level, color::Fg(color::Reset), msg);
        let _ = match log_level {
            LogLevel::Error => writeln!(io::stderr(), "{}", String::from_utf8(writer).unwrap()),
            _ => writeln!(io::stdout(), "{}", String::from_utf8(writer).unwrap()),
        };
   }
}
