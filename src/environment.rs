//      imports
//      =======
use crate::errors::EnvironmentParseError;

use std::{
    env,
    fmt::{self, Write},
    fs::metadata,
    path::PathBuf,
    process::exit,
};

//      structures
//      ==========
#[derive(Debug)]
pub struct Environment {
    pub color: bool,
    pub time: String,
    pub source_dir: PathBuf,
    pub address: [u8; 4],
    pub port: u16,
    pub timeout: u64,
}

//      impl(s)
//      =======
impl Default for Environment {
    fn default() -> Self {
        Self {
            color: false,
            time: String::from(""),
            source_dir: PathBuf::from("./html/"),
            address: [127, 0, 0, 1],
            port: 6969,
            timeout: 5000,
        }
    }
}

// --------
impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\tcolor:\t\t{}", self.color)?;
        writeln!(f, "\ttime:\t\t{}", self.time)?;
        writeln!(f, "\tsource_dir:\t{:?}", self.source_dir)?;
        writeln!(f, "\taddress:\t{:?}", self.address)?;
        writeln!(f, "\tport:\t\t{:?}", self.port)?;
        writeln!(f, "\ttimeout:\t{:?}", self.timeout)
    }
}

// --------
// private functions
impl Environment {
    fn print_help() {
        let mut output = String::new();
        let _ = writeln!(output, "Usage:");
        let _ = writeln!(output, "\tsert [options]");
        let _ = writeln!(output, "\nLOGGER OPTIONS");
        let _ = writeln!(
            output,
            "\t-c, --color, --colour\tuses colors when formatting different log output"
        );
        let _ = writeln!(
            output,
            "\t-t, --time\t\tspecifies the date-time formatting to be used. leave blank for no date-time information. see `man time` for valid formatting"
        );

        let _ = writeln!(output, "\nSERVER OPTIONS");
        let _ = writeln!(
            output,
            "\t-p, --path\t\tspecifies the directory to use to look for the html files"
        );
        let _ = writeln!(
            output,
            "\t-a, --address\t\tspecifies the address to use to host the server"
        );
        let _ = writeln!(
            output,
            "\t-T, --timeout\t\tspecifies the amount of milliseconds the program should wait for the listener thread before terminating"
        );
        let _ = writeln!(output, "\t-P, --port\t\tspecifies the port to listen on");

        println!("{}", output);
    }

    fn process_port(port: String) -> Result<u16, EnvironmentParseError> {
        match port.parse::<u16>() {
            Ok(port) => Ok(port),
            Err(_) => return Err(EnvironmentParseError::InvalidPort(port.to_string())),
        }
    }

    fn process_address(&mut self, input: String) -> Result<(), EnvironmentParseError> {
        let addr = input.split(":").collect::<Vec<_>>();
        match addr.len() {
            1 => (),
            2 => match Self::process_port(addr[1].to_string()) {
                Ok(port) => self.port = port,
                Err(err) => return Err(err),
            },
            _ => return Err(EnvironmentParseError::InvalidAddr(input)),
        }

        let addr = addr[0].split(".").collect::<Vec<_>>();
        if addr.len() != 4 {
            return Err(EnvironmentParseError::InvalidAddr(input));
        }

        let mut count = 0;
        for element in addr.into_iter() {
            match element.parse::<u8>() {
                Ok(num) => {
                    self.address[count] = num;
                    count += 1;
                }
                Err(_) => return Err(EnvironmentParseError::InvalidAddr(input)),
            }
        }
        Ok(())
    }

    fn process_path(path: String) -> Result<PathBuf, EnvironmentParseError> {
        // TODO: check if directory can be read from?
        let pb = PathBuf::from(path.clone());
        // check if it exists
        let md = match metadata(pb.as_path()) {
            Ok(md) => md,
            Err(_) => return Err(EnvironmentParseError::InvalidPath(path)),
        };

        // check if its a directory
        if !md.is_dir() {
            return Err(EnvironmentParseError::NotADir(path));
        }

        Ok(pb)
    }
}

// --------
// public functions
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
                        default.source_dir = match Self::process_path(path.to_string()) {
                            Ok(path) => path,
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-a" | "--address" => {
                    if let Some(x) = args.next() {
                        match default.process_address(x.to_string()) {
                            Ok(()) => (),
                            Err(x) => {
                                return Err(x);
                            }
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-P" | "--port" => {
                    if let Some(port) = args.next() {
                        match Self::process_port(port.to_string()) {
                            Ok(port) => default.port = port,
                            Err(err) => return Err(err),
                        }
                    }
                }

                "-T" | "--timeout" => {
                    if let Some(timeout) = args.next() {
                        default.timeout = match timeout.parse::<u64>() {
                            Ok(number) => number,
                            Err(_) => return Err(EnvironmentParseError::InvalidTimeout(timeout)),
                        }
                    }
                }

                "-h" | "--help" => {
                    Self::print_help();
                    exit(0);
                }

                _ => return Err(EnvironmentParseError::InvalidArg(i)),
            }
        }

        return Ok(default);
    }
}
