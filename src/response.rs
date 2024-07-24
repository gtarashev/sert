use crate::{
    environment::Environment,
    log::{LogLevel, Logger},
    request::{HttpMethod, Request},
};

use std::{fs, io::Write, net::TcpStream};

pub fn handle_client(env: &Environment, logger: &Logger, stream: TcpStream) {
    let mut request = match Request::try_from(stream) {
        Ok(request) => request,
        Err(_) => {
            logger.log(LogLevel::Error, "Error parsing TCP stream.");
            return;
        }
    };
    logger.log(LogLevel::Info, format!("{:#?}", request));

    let mut status = match request.method {
        HttpMethod::GET => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 NOT FOUNT",
    };

    let file = match &request.content[..] {
        "/" | "/test.html" | "test.html" => "test.html",
        _ => {
            status = "HTTP/1.1 404 NOT FOUNT";
            "html/not_found.html"
        }
    };

    let contents = match fs::read_to_string(format!("{}{}", env.source_dir.to_str().unwrap(), file))
    {
        Ok(x) => x,
        Err(e) => {
            logger.log(
                LogLevel::Error,
                format!("Error opening {:?}/{}: {}", env.source_dir, file, e),
            );
            String::from(
                r#"<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="utf-8">
                        <title>Not found</title>
                    </head>
                    <body>
                        <h1>Oops!</h1>
                        <p>Sorry, I don't know what you're asking for.</p>
                    </body>
                </html>"#,
            )
        }
    };
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
