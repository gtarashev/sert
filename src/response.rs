use crate::{
    log::{LogLevel, Logger},
    request::{HttpMethod, Request},
};

use std::{fs, io::Write, net::TcpStream};

pub fn handle_client(logger: &Logger, stream: TcpStream) {
    let mut request = match Request::try_from(stream) {
        Ok(request) => request,
        Err(_) => {
            logger.log(LogLevel::Error, "Error parsing TCP stream.");
            return;
        }
    };
    logger.log(LogLevel::Info, format!("{:#?}", request));

    let status = match request.method {
        HttpMethod::GET => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 NOT FOUNT",
    };

    let file = match &request.content[..] {
        "/" | "/test.html" | "test.html" => "html/test.html",
        _ => "html/not_found.html",
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, contents
    );

    match request.writer.write(response.as_bytes()) {
        Ok(_) => logger.log(LogLevel::Info, "Client served successfully"),
        Err(_) => logger.log(LogLevel::Error, "Error sending data."),
    }
}
