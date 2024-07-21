use crate::errors::MethodError;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub enum HttpMethod {
    POST,
    GET,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub content: String,
    pub writer: TcpStream,
}

impl TryFrom<TcpStream> for Request {
    type Error = MethodError;
    fn try_from(mut tcp_stream: TcpStream) -> Result<Self, Self::Error> {
        let buf_reader = BufReader::new(&mut tcp_stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        if http_request.len() == 0 {
            return Err(MethodError::new(String::from("error setting something")));
        }

        let request_line = http_request[0].split(" ").collect::<Vec<_>>();
        if request_line.len() != 3 {
            return Err(MethodError::new(String::from("error setting something")));
        }
        let (method, content);
        method = match request_line[0] {
            "POST" => HttpMethod::POST,
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "CONNECT" => HttpMethod::CONNECT,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "PATCH" => HttpMethod::PATCH,
            x => return Err(MethodError::new(String::from(x))),
        };

        content = String::from(request_line[1]);

        let writer = tcp_stream;
        return Ok(Request {
            method,
            content,
            writer,
        });
    }
}
