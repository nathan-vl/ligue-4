use std::{
    io::{stdin, stdout, Result, Write},
    net::TcpStream,
};

use crate::{request::Request, response::Response, server::PORT, tile::Tile, utils};

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
        self.handle_response(&response);
    }

    pub fn play(&mut self) {
        'game: loop {
            let response = self.read_response();
            self.handle_response(&response);

            match response {
                Response::PlayerWin { board: _ }
                | Response::PlayerLost { board: _ }
                | Response::OtherPlayerDisconnected => break 'game,
                _ => {}
            }
        }
    }

    fn handle_response(&mut self, response: &Response) {
        match response {
            Response::CreatedRoom => {
                println!("Sala criada. Aguardando outro jogador.");
                self.tile = Some(Tile::Player1);

                // Aguardando jogador 2
                let response = self.read_response();
                self.handle_response(&response);
            }
            Response::JoinedRoom { player_tile } => {
                println!(
                    "Entrou na sala. Você é o jogador {}.",
                    player_tile.to_number()
                );
                self.tile = Some(Tile::Player2);
            }
            Response::AnotherPlayerJoinedRoom { player_tile } => {
                println!(
                    "Outro jogador entrou na sala. Você é o jogador {}.",
                    player_tile.to_number()
                );
            }
            Response::AskTurn { board } => {
                board.print();

                print!("É a sua vez, escolha uma coluna de 1 a 7: ");
                let _ = stdout().flush();

                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                let col = s.trim().parse::<i32>().unwrap() - 1;

                self.send_request(Request::Play { column: col as u8 })
                    .unwrap();
            }
            Response::AnotherPlayerTurn { board } => {
                board.print();
                println!("Aguardando o outro jogador");
            }
            Response::PlayerWin { board } => {
                board.print();
                println!("Fim de jogo. Você ganhou.");
            }
            Response::PlayerLost { board } => {
                board.print();
                println!("Fim de jogo. Você perdeu.");
            }
            Response::Draw { board } => {
                board.print();
                println!("Fim de jogo. Foi um empate.");
            }
            Response::InvalidRequest { message } => {
                println!("Requisição inválida: {message}");
            }
            Response::OtherPlayerDisconnected => {
                println!("O outro jogador saiu da sala.");
            }
        }
    }

    fn send_request(&mut self, request: Request) -> Result<()> {
        utils::send(&mut self.stream, &request)
    }

    fn read_response(&mut self) -> Response {
        utils::read(&mut self.stream).unwrap()
    }
}
