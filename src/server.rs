use std::net::{Shutdown, TcpListener, TcpStream};

use crate::{
    game::Game,
    game_room::GameRoom,
    request::Request,
    response::Response,
    tile::Tile,
    utils::{self},
};

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

    fn handle_game(mut player1: TcpStream, mut player2: TcpStream) {
        println!("Jogo iniciou");
        let mut game = Game::new();

        // TODO: Check draw
        loop {
            let current_player_request = if game.current_player == Tile::Player1 {
                Server::send_response(
                    &mut player2,
                    Response::AnotherPlayerTurn { board: game.board },
                );

                Server::send_response(&mut player1, Response::AskTurn { board: game.board });
                Server::read_request(&mut player1)
            } else {
                Server::send_response(
                    &mut player1,
                    Response::AnotherPlayerTurn { board: game.board },
                );

                Server::send_response(&mut player2, Response::AskTurn { board: game.board });
                Server::read_request(&mut player2)
            };

            match current_player_request {
                Request::NewPlayer => panic!("Invalid request"),
                Request::Play { column } => {
                    if let Some(tile_pos) =
                        game.board.place_tile(column as usize, &game.current_player)
                    {
                        if game
                            .board
                            .check_win(&game.current_player, tile_pos.0, tile_pos.1)
                        {
                            if game.current_player == Tile::Player1 {
                                Self::send_response(
                                    &mut player1,
                                    Response::PlayerWin { board: game.board },
                                );
                                Self::send_response(
                                    &mut player2,
                                    Response::PlayerLost { board: game.board },
                                );
                            } else {
                                Self::send_response(
                                    &mut player1,
                                    Response::PlayerLost { board: game.board },
                                );
                                Self::send_response(
                                    &mut player2,
                                    Response::PlayerWin { board: game.board },
                                );
                            }

                            _ = player1.shutdown(Shutdown::Both);
                            _ = player2.shutdown(Shutdown::Both);
                            println!("Jogo acabou");
                            return;
                        } else if game.board.check_tie(){
                            
                            Self::send_response(
                                &mut player1,
                                Response::Draw {board: game.board}
                            );
                            Self::send_response(
                                &mut player2,
                                Response::Draw {board: game.board}
                            );

                            _ = player1.shutdown(Shutdown::Both);
                            _ = player2.shutdown(Shutdown::Both);
                            println!("Jogo empatou, ninguém venceu");
                            return;
                        }
                    } else {
                        panic!("Invalid position");
                    }
                }
            };

            game.current_player = game.current_player.opposite();
        }
    }

    fn handle_new_player(mut stream: TcpStream, rooms: &mut Vec<GameRoom>) {
        let available_room = GameRoom::first_available_room(rooms);
        match available_room {
            Some(mut room) => {
                room.player2 = Some(stream);
                room.game = Some(Game::new());

                Server::send_response(
                    &mut room.player1,
                    Response::AnotherPlayerJoinedRoom {
                        player_tile: Tile::Player1,
                    },
                );
                Server::send_response(
                    room.player2.as_mut().unwrap(),
                    Response::JoinedRoom {
                        player_tile: Tile::Player2,
                    },
                );

                Server::handle_game(room.player1, room.player2.unwrap());
            }
            None => {
                Server::send_response(&mut stream, Response::CreatedRoom);
                let room = GameRoom::new(stream);
                rooms.push(room);
            }
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
