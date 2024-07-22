use std::env;
use std::process::exit;
use std::fmt::Write;
use crate::errors::EnvironmentParseError;

pub struct Environment {
    pub color: bool,
    pub time: String,
}

fn print_help() {
    let mut output = String::new();
    let _ = writeln!(output, "Usage:");
    let _ = writeln!(output, "\tsert [options]");

    println!("{}", output);
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            color: false,
            time: String::from(""),
        }
    }
}

impl Environment {
    pub fn from_args() -> Result<Self, EnvironmentParseError> {
        let mut default = Self::default();
        let mut args = env::args();
        if args.len() == 1 {
            return Ok(default);
        }

        // first arg is the name of the program
        let _ = args.next();

        while let Some(i) = args.next() {
            match &i[..] {
                "-c"|"--color"|"--colour" => default.color = true,

                "-t"|"--time" => {
                    if let Some(time_format) = args.next() {
                        default.time = time_format;
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-h"|"--help" => {
                    print_help();
                    exit(0);
                }

                _ => return Err(EnvironmentParseError::InvalidArg(i)),
            }
        }

        return Ok(default);
    }
}
