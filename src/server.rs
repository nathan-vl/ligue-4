use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::{game_room::GameRoom, request::Request};

pub const PORT: u16 = 6010;

pub struct Server {
    listener: TcpListener,
    rooms: Vec<GameRoom>,
}

impl Server {
    pub fn new() -> Self {
        let addr = format!("0.0.0.0:{PORT}");
        Self {
            listener: TcpListener::bind(&addr).unwrap(),
            rooms: vec![],
        }
    }

    pub fn listen(&mut self) {
        println!("Listening at {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            Server::handle_connection(&mut stream);
        }
    }

    fn handle_connection(connection: &mut TcpStream) {
        println!("---\nIncoming ---");
        let request = Server::read_request(connection);
        println!("{:?}", request);
        println!("--- End ---");
    }

    fn read_request(connection: &mut TcpStream) -> Request {
        let mut buf: [u8; 1024] = [0; 1024];
        connection.read(&mut buf).unwrap();

        serde_json::from_value(
            serde_json::to_value(&String::from_utf8(buf.to_vec()).unwrap()).unwrap(),
        )
        .unwrap()
    }
}
