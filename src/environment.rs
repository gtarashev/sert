use crate::errors::EnvironmentParseError;
use std::env;
use std::fmt;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
pub struct Environment {
    pub color: bool,
    pub time: String,
    pub source_dir: PathBuf,
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
            source_dir: PathBuf::from(""),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\tcolor:\t{}", self.color)?;
        writeln!(f, "\ttime:\t{}", self.time)
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
                "-c" | "--color" | "--colour" => default.color = true,

                "-t" | "--time" => {
                    if let Some(time_format) = args.next() {
                        default.time = time_format;
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-p" | "--path" => {
                    if let Some(path) = args.next() {
                        default.source_dir = PathBuf::from(path.clone());
                        // check if path exists
                        if !default.source_dir.as_path().exists() {
                            return Err(EnvironmentParseError::InvalidPath(path));
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-h" | "--help" => {
                    print_help();
                    exit(0);
                }

                _ => return Err(EnvironmentParseError::InvalidArg(i)),
            }
        }

        return Ok(default);
    }
}
