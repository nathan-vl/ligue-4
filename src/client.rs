use std::{io::{Result, Write}, net::TcpStream};

use crate::{request::Request, server::PORT};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(server_ip: &str) -> Self {
        Self {
            stream: TcpStream::connect(format!("{server_ip}:{PORT}")).unwrap(),
        }
    }

    pub fn join_game(&mut self) {
        self.send_request(&Request::New).unwrap();
    }

    fn send_request(&mut self, request: &Request) -> Result<()> {
        let request_str = serde_json::to_string(request)?;
        println!("--- SEND:{request_str}");
        self.stream.write(request_str.as_bytes())?;
        Ok(())
    }
}
