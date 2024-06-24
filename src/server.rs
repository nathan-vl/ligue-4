use std::{
    io::Result,
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

use rand::Rng;

use crate::{
    game::Game, game_room::GameRoom, player::Player, request::Request, response::Response,
    tile::Tile, utils,
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
        let mut game = Game::new();

        let mut rng = rand::thread_rng();
        let decide_starting_player: u8 = rng.gen_range(1..3);
        if decide_starting_player == 2 {
            game.current_player = Tile::Player2;
        }

        loop {
            let (current_player, other_player) = if game.current_player == Tile::Player1 {
                (&mut player1, &mut player2)
            } else {
                (&mut player2, &mut player1)
            };

            Server::send_response(
                other_player,
                Response::AnotherPlayerTurn { board: game.board },
            )
            .unwrap();

            Server::send_response(current_player, Response::AskTurn { board: game.board }).unwrap();
            let current_player_request = Server::read_request(current_player);

            match current_player_request {
                Ok(request) => match request {
                    Request::Play { column } => {
                        if let Some(tile_pos) =
                            game.board.place_tile(column as usize, &game.current_player)
                        {
                            if game
                                .board
                                .check_win(&game.current_player, tile_pos.0, tile_pos.1)
                            {
                                Self::send_response(
                                    current_player,
                                    Response::PlayerWin { board: game.board },
                                )
                                .unwrap();
                                Self::send_response(
                                    other_player,
                                    Response::PlayerLost { board: game.board },
                                )
                                .unwrap();

                                _ = player1.shutdown(Shutdown::Both);
                                _ = player2.shutdown(Shutdown::Both);
                                println!("Jogo acabou");
                                return;
                            } else if game.board.check_tie() {
                                Self::send_response(
                                    &mut player1,
                                    Response::Draw { board: game.board },
                                )
                                .unwrap();
                                Self::send_response(
                                    &mut player2,
                                    Response::Draw { board: game.board },
                                )
                                .unwrap();

                                _ = player1.shutdown(Shutdown::Both);
                                _ = player2.shutdown(Shutdown::Both);
                                println!("Jogo empatou, ninguém venceu");
                                return;
                            }
                        } else {
                            panic!("Invalid position");
                        }
                    }
                    _ => panic!("Requisição inválida"),
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        _ = Server::send_response(other_player, Response::OtherPlayerDisconnected);
                        return;
                    }
                }
            };

            game.current_player = game.current_player.opposite();
        }
    }

    fn handle_new_player(mut player: Player, rooms: &mut Vec<GameRoom>) {
        let available_room = GameRoom::first_available_room(rooms);
        match available_room {
            Some(mut room) => {
                room.player2 = Some(player);
                room.game = Some(Game::new());

                Server::send_response(
                    &mut room.player1.stream,
                    Response::AnotherPlayerJoinedRoom {
                        player_tile: Tile::Player1,
                        another_player_name: room.player2.as_mut().unwrap().name.clone(),
                    },
                )
                .unwrap();
                Server::send_response(
                    &mut room.player2.as_mut().unwrap().stream,
                    Response::JoinedRoom {
                        player_tile: Tile::Player2,
                        other_player_name: room.player1.name,
                    },
                )
                .unwrap();

                thread::spawn(|| {
                    Server::handle_game(room.player1.stream, room.player2.unwrap().stream);
                });
            }
            None => {
                Server::send_response(&mut player.stream, Response::CreatedRoom).unwrap();
                let room = GameRoom::new(player);
                rooms.push(room);
            }
        }
    }

    pub fn listen(&mut self) {
        println!(
            "Servidor ligado no endereço {}.",
            self.listener.local_addr().unwrap()
        );
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = Server::read_request(&mut stream).unwrap();

            match request {
                Request::NewPlayer { name } => {
                    Server::handle_new_player(Player::new(name, stream), &mut self.rooms);
                }
                _ => Server::send_response(
                    &mut stream,
                    Response::InvalidRequest {
                        message: "É necessário entrar em uma sala para fazer uma jogada".to_owned(),
                    },
                )
                .unwrap(),
            }
        }
    }

    fn read_request(stream: &mut TcpStream) -> Result<Request> {
        utils::read(stream)
    }

    fn send_response(stream: &mut TcpStream, response: Response) -> Result<()> {
        utils::send(stream, &response)
    }
}
