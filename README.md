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

---
## Dev
### Branches
There are 2 current "stable" branches, `main` and `stable`. There may also be other "feature" branches (with their name corresponding to the feature that they are created for to be implemented). A given commit on a feature branch may not be formatter, or even compile. Once a feature is complete the feature branch will be merged into `stable`. `stable` will contain code that is formatted and buildable, however, it may contain code that includes rust implementations that should not be used in an environment that shouldn't panic (such as `unwrap()`). Once a feature is implemented, the code is formatted, and there are no runtime panic implementations, `stable` will be merged into `main`. Since I've decided to implement this model in the middle of development, previous `main` commits may not follow this style.
