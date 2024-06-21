use std::{
    io::{stdin, stdout, Result, Write},
    net::TcpStream,
};

use crate::{board::Board, request::Request, response::Response, server::PORT, tile::Tile, utils};

pub struct Client {
    stream: TcpStream,
    tile: Option<Tile>,
}

impl Client {
    pub fn new(server_ip: &str) -> Self {
        Self {
            stream: TcpStream::connect(format!("{server_ip}:{PORT}")).unwrap(),
            tile: None,
        }
    }

    pub fn join_game(&mut self) {
        self.send_request(Request::NewPlayer).unwrap();
        let response = self.read_response();
        self.handle_response(response);
    }

    pub fn play(&mut self) {
        let response = self.read_response();
        self.handle_response(response);
    }

    fn display_board_state(&mut self, board: &Board, current_player: Tile) {
        let player_tile = self.tile.unwrap();
        let is_current_player = player_tile == current_player;
        print!("Você é o jogador {}. ", player_tile.to_number());
        if is_current_player {
            println!("É a sua vez");
        } else {
            println!("É a vez do jogador {}.", player_tile.opposite().to_number());
        }
        board.print();
    }

    fn handle_current_state(&mut self, board: &Board, current_player: Tile) {
        self.display_board_state(board, current_player);
        let is_current_player = self.tile.unwrap() == current_player;
        if is_current_player {
            print!(
                "Jogador {}, escolha uma coluna de 1 a 7: ",
                if current_player == Tile::Player1 {
                    1
                } else {
                    2
                }
            );
            let _ = stdout().flush();

            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            let col = s.trim().parse::<i32>().unwrap() - 1;

            self.send_request(Request::Play { column: col as u8 })
                .unwrap();
        }
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::PlayerCreatedRoom => {
                println!("Você é o jogador 1. Aguardando jogador 2.");
                self.tile = Some(Tile::Player1);

                // Aguardando jogador 2
                let response = self.read_response();
                self.handle_response(response);
            }
            Response::PlayerEnteredRoom { board } => {
                println!("Você é o jogador 2.");
                self.tile = Some(Tile::Player2);
                self.handle_current_state(&board, Tile::Player1);
            }
            Response::AnotherPlayerJoinedRoom { board } => {
                println!("O jogador 2 entrou na sala.");
                self.handle_current_state(&board, Tile::Player1);
            }
            Response::CurrentGameState {
                board,
                current_player,
            } => self.handle_current_state(&board, current_player),
        }
    }

    fn send_request(&mut self, request: Request) -> Result<()> {
        utils::send(&mut self.stream, &request)
    }

    fn read_response(&mut self) -> Response {
        utils::read(&mut self.stream).unwrap()
    }
}
