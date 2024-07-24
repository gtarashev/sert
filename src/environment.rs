use crate::errors::EnvironmentParseError;
use std::env;
use std::fmt;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;
use std::fs::metadata;

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
            source_dir: PathBuf::from("./html/"),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\tcolor:\t{}", self.color)?;
        writeln!(f, "\ttime:\t{}", self.time)?;
        writeln!(f, "\tsource_dir:\t{:?}", self.source_dir)
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
                        // check if the path is a directory
                        let md = match metadata(default.source_dir.as_path()) {
                            Ok(md) => md,
                            Err(err) => {
                                println!("error: {:#?}", err);
                                exit(1);
                            }
                        };

                        if !md.is_dir() {
                            return Err(EnvironmentParseError::NotADir(path));
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
