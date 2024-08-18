//      imports
//      =======
use crate::errors::EnvironmentParseError;

use std::{
    fmt::{self, Write},
    fs::{metadata, File},
    io::Read,
    path::PathBuf,
    process::exit,
};

//      structures
//      ==========
#[derive(Debug, Clone)]
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

        let _ = writeln!(output, "\t-C --config-file\tuse a configuration file for the server. The config file may overwrite previously set command line options. On the other hand, you can overwrite config file options if you set them after giving the config file.");

        eprint!("{}", output);
    }

    fn process_port(&mut self, port: String) -> Result<(), EnvironmentParseError> {
        match port.parse::<u16>() {
            Ok(port) => self.port = port,
            Err(_) => return Err(EnvironmentParseError::InvalidPort(port.to_string())),
        }

        Ok(())
    }

    fn process_address(&mut self, input: String) -> Result<(), EnvironmentParseError> {
        let addr = input.split(":").collect::<Vec<_>>();
        match addr.len() {
            1 => (),
            2 => match self.process_port(addr[1].to_string()) {
                Ok(()) => (),
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

    fn process_path(&mut self, path: String) -> Result<(), EnvironmentParseError> {
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

        self.source_dir = pb;
        Ok(())
    }

    fn process_color(&mut self, option: Option<String>) -> Result<(), EnvironmentParseError> {
        let c = match option {
            Some(to_parse) => match &(to_parse.to_lowercase())[..] {
                "true" => true,
                "false" => false,
                _ => {
                    return Err(EnvironmentParseError::InvalidOption(
                        String::from("color"),
                        to_parse,
                    ))
                }
            },
            None => true,
        };

        self.color = c;

        Ok(())
    }

    fn process_timeout(&mut self, timeout: String) -> Result<(), EnvironmentParseError> {
        self.timeout = match timeout.parse::<u64>() {
            Ok(number) => number,
            Err(_) => return Err(EnvironmentParseError::InvalidTimeout(timeout)),
        };

        Ok(())
    }

    fn process_time(&mut self, time: String) -> Result<(), EnvironmentParseError> {
        self.time = time;
        Ok(())
    }
}

// --------
// public functions
impl Environment {
    pub fn from_args<T: ExactSizeIterator<Item = String>>(
        mut default: Self,
        mut args: T,
    ) -> Result<Self, EnvironmentParseError> {
        if args.len() == 0 {
            return Ok(default);
        }

        while let Some(i) = args.next() {
            match &i[..] {
                "-c" | "--color" | "--colour" => match default.process_color(args.next()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                "-t" | "--time" => {
                    if let Some(time_format) = args.next() {
                        match default.process_time(time_format) {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-p" | "--path" => {
                    if let Some(path) = args.next() {
                        match default.process_path(path) {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-a" | "--address" => {
                    if let Some(addr) = args.next() {
                        match default.process_address(addr) {
                            Ok(_) => (),
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
                        match default.process_port(port) {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-T" | "--timeout" => {
                    if let Some(timeout) = args.next() {
                        match default.process_timeout(timeout) {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(EnvironmentParseError::NullArg(i));
                    }
                }

                "-h" | "--help" => {
                    Self::print_help();
                    exit(0);
                }

                "-C" | "--config-file" => {
                    if let Some(filename) = args.next() {
                        default = match Self::from_config(default, filename) {
                            Ok(e) => e,
                            Err(err) => return Err(err),
                        }
                    }
                }

                _ => return Err(EnvironmentParseError::InvalidArg(i)),
            }
        }

        return Ok(default);
    }

    pub fn from_config(mut default: Self, filename: String) -> Result<Self, EnvironmentParseError> {
        let mut file = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => {
                return Err(EnvironmentParseError::ConfigFileError(format!(
                    "Could not open file: {}",
                    filename
                )));
            }
        };

        let mut lines = String::new();
        match file.read_to_string(&mut lines) {
            Ok(_) => (),
            Err(err) => {
                return Err(EnvironmentParseError::ConfigFileError(format!(
                    "Error reading file: {}",
                    err
                )))
            }
        }

        let mut line_num = 0;
        for line in lines.lines() {
            line_num += 1;
            // comments start with #
            if line.starts_with("#") {
                continue;
            }
            // the split might interrupt if there are values which contain `=`
            // one way to combat this would be split_once, but that is a nightly
            // only function, so i can just join all the key arguments back
            let split = line.split("=").collect::<Vec<_>>();
            if split.len() < 2 {
                return Err(EnvironmentParseError::ConfigFileError(format!(
                    "Incorrect formatting on line {}",
                    line_num
                )));
            }
            let key = split[0];
            let value = split[1..].join("");

            match key.trim() {
                "color" | "colour" => match default.process_color(Some(value.into())) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                "time" => match default.process_time(value.into()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                "path" => match default.process_path(value.into()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                "address" => match default.process_address(value.into()) {
                    Ok(_) => (),
                    Err(x) => {
                        return Err(x);
                    }
                },

                "port" => match default.process_port(value.into()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                "timeout" => match default.process_timeout(value.into()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },

                _ => return Err(EnvironmentParseError::InvalidConfigKey(key.into())),
            }
        }

        return Ok(default);
    }
}
