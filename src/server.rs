use std::net::{TcpListener, TcpStream};

use crate::{game::Game, game_room::GameRoom, request::Request, response::Response, utils};

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

    fn handle_new_player(stream: TcpStream, rooms: &mut Vec<GameRoom>) {
        let mut available_rooms = GameRoom::available_rooms(rooms);
        if available_rooms.is_empty() {
            let mut room: GameRoom = GameRoom::new(stream);
            Server::send_response(&mut room.player1, Response::PlayerCreatedRoom);
            rooms.push(room);
        } else {
            let room = &mut available_rooms[0];
            room.player2 = Some(stream);

            room.game = Some(Game::new());

            let board = &room.game.as_ref().unwrap().board;

            Server::send_response(
                &mut room.player1,
                Response::AnotherPlayerJoinedRoom { board: board.clone() },
            );
            Server::send_response(
                room.player2.as_mut().unwrap(),
                Response::PlayerEnteredRoom { board: board.clone() },
            );
        }
    }

    pub fn listen(&mut self) {
        println!("Listening at {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = Server::read_request(&mut stream);

            match request {
                Request::NewPlayer => Server::handle_new_player(stream, &mut self.rooms),
                Request::Play { column: _ } => todo!("Server REQ Play"),
            }
        }
    }

    fn read_request(stream: &mut TcpStream) -> Request {
        utils::read(stream).unwrap()
    }

    fn send_response(stream: &mut TcpStream, response: Response) {
        utils::send(stream, &response).unwrap();
    }
}
