use std::{
    io::Result,
    net::{TcpListener, TcpStream},
};

use crate::{game_room::GameRoom, request::Request, response::Response, utils};

pub const PORT: u16 = 6010;

pub struct Server {
    listener: TcpListener,
    rooms: Vec<GameRoom>,
}

impl Server {
    pub fn new() -> Self {
        let addr = format!("0.0.0.0:{PORT}");
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            rooms: vec![],
        }
    }

    pub fn listen(&mut self) {
        println!("Listening at {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = Server::read_request(&mut stream);

            match request {
                Request::NewPlayer => {
                    let mut available_rooms = GameRoom::available_rooms(&mut self.rooms);
                    if available_rooms.is_empty() {
                        Server::send_response(
                            &mut stream,
                            &Response::JoinedRoom {
                                message: "Created room".to_owned(),
                            },
                        );

                        let room = GameRoom::new(stream);
                        self.rooms.push(room);
                    } else {
                        Server::send_response(
                            &mut stream,
                            &Response::JoinedRoom {
                                message: "Joined room".to_owned(),
                            },
                        );

                        available_rooms[0].player2 = Some(stream);
                    }
                }
                Request::Play { column: _ } => todo!(),
            }
        }
    }

    fn read_request(stream: &mut TcpStream) -> Request {
        utils::read(stream).unwrap()
    }

    fn send_response(stream: &mut TcpStream, response: &Response) {
        utils::send(stream, response).unwrap();
    }
}
