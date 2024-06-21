use std::{io::Result, net::TcpStream};

use crate::{request::Request, response::Response, server::PORT, utils};

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
        self.send_request(&Request::NewPlayer).unwrap();
        let response = self.read_response();
        println!("{:?}", response);
    }

    fn send_request(&mut self, request: &Request) -> Result<()> {
        utils::send(&mut self.stream, request)
    }

    fn read_response(&mut self) -> Response {
        utils::read(&mut self.stream).unwrap()
    }
}
