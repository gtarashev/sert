# A simple web-server in Rust

---

## Functionality:
### Server
- process an incoming request and turn it into an object
- handle said requests (for now only `GET`)
- send a http response from a html file
- handling of http requests is multi-threaded

### Log
- print to stdout and stderr with logging information
- different log levels 
- optional date formatting
- optional color formatting

---
## Building and running

Simply run `cargo r` to run the server with some defaults set. If you want to build you can either build it in debug mode with `cargo b` or in release mode (faster but no stack trace) with `cargo b --release`. Both building and running will build the binary and put it into `./target/{build_mode}/`. For usage, run `sert -h`.
