use std::{
    io::Result,
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

use std::io;

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
        let mut rematch = false;

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

            match Server::send_response(
                other_player,
                Response::AnotherPlayerTurn { board: game.board },
            ) {
                Ok(_) => {}
                Err(e) => {
                    println!("Erro ao enviar o tabuleiro para o outro jogador: {:?}", e);
                    return;
                }
            }

            let mut current_player_request;
            let chosen_column: u8;
            loop {
                Server::send_response(
                    current_player, 
                    Response::AskTurn { board: game.board }
                ).unwrap();
                
                current_player_request = match Server::read_request(current_player) {
                    Ok(request) => request,
                    Err(e) => {
                        println!("Erro ao ler a solicitação do jogador: {:?}", e);
                        continue;
                    }
                }; 

                if let Request::Play { column } = current_player_request {
                    if game.board.is_column_within_bounds(column.into()) {
                        if !game.board.is_column_full(column.into()) {
                            chosen_column = column;
                            break;
                        } else {
                            Server::send_response(current_player, Response::InvalidColumn {
                                message: "Coluna cheia. Escolha outra.".to_owned(),
                            }).unwrap();
                        }
                    } else {
                        Server::send_response(current_player, Response::InvalidColumn {
                            message: "Escolha uma coluna de 1 a 7.".to_owned(),
                        }).unwrap();
                    }
                } else {
                    Server::send_response(current_player, Response::InvalidRequest {
                        message: "Apenas a solicitação Play é válida. Tente novamente.".to_owned(),
                    }).unwrap();
                }
            }

            if let Some(tile_pos) =
            game.board.place_tile(chosen_column as usize, &game.current_player)
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

                    match Server::check_for_rematch(current_player, other_player) {
                        Ok(wants_rematch) => {
                            if wants_rematch {
                                rematch = true;
                            } else {
                                println!("Pelo menos um dos jogadores não quis revanche");
                                _ = player1.shutdown(Shutdown::Both);
                                _ = player2.shutdown(Shutdown::Both);
                                println!("Jogo acabou");
                                return;
                            }
                        }
                        Err(e) => {
                            println!("Erro ao verificar revanche: {:?}", e);
                            return;
                        }
                    }                          
                } else if game.board.check_tie() {
                    Self::send_response(
                        current_player,
                        Response::Draw { board: game.board },
                    )
                    .unwrap();
                    Self::send_response(
                        other_player,
                        Response::Draw { board: game.board },
                    )
                    .unwrap();

                    match Server::check_for_rematch(current_player, other_player) {
                        Ok(wants_rematch) => {
                            if wants_rematch {
                                rematch = true;
                            } else {
                                println!("Pelo menos um dos jogadores não quis revanche");
                                _ = player1.shutdown(Shutdown::Both);
                                _ = player2.shutdown(Shutdown::Both);
                                println!("Jogo acabou");
                                return;
                            }
                        }
                        Err(e) => {
                            println!("Erro ao verificar revanche: {:?}", e);
                            return;
                        }
                    } 
                }
            } else {
                panic!("Invalid position");
            }                            
            

            game.current_player = game.current_player.opposite();
            if rematch {
                break;
            }
        }
        Server::handle_game(player1, player2);
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

    fn check_for_rematch(player1: &mut TcpStream, player2: &mut TcpStream) -> io::Result<bool> {
        if let Err(e) = Self::send_response(player1, Response::Rematch) {
            println!(
                "Erro ao enviar resposta de revanche para o jogador 1: {:?}",
                e
            );
            return Err(e);
        }
        let player1_response = Server::read_request(player1)?;
        let player1_rematch = match player1_response {
            Request::Rematch { accept } => accept == "S" || accept == "s",
            _ => panic!("Resposta inválida do jogador 1"),
        };

        if let Err(e) = Self::send_response(player2, Response::Rematch) {
            println!(
                "Erro ao enviar resposta de revanche para o jogador 2: {:?}",
                e
            );
            return Err(e);
        }
        let player2_response = Server::read_request(player2)?;
        let player2_rematch = match player2_response {
            Request::Rematch { accept } => accept == "S" || accept == "s",
            _ => panic!("Resposta inválida do jogador 2"),
        };

        if player1_rematch && player2_rematch {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn read_request(stream: &mut TcpStream) -> Result<Request> {
        utils::read(stream)
    }

    fn send_response(stream: &mut TcpStream, response: Response) -> Result<()> {
        utils::send(stream, &response)
    }
}
