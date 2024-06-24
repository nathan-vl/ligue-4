use std::{
    io::{stdin, stdout, Result, Write},
    net::TcpStream,
    process::exit,
};

use crate::{request::Request, response::Response, server::PORT, utils};

pub struct Client {
    stream: TcpStream,
    player_name: String,
}

impl Client {
    pub fn new(player_name: String, server_ip: &str) -> Self {
        let stream = TcpStream::connect(format!("{server_ip}:{PORT}"));
        match stream {
            Ok(stream) => Self {
                stream,
                player_name,
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    println!("Não foi possível se conectar ao servidor.");
                    exit(-1);
                } else {
                    panic!("{e}");
                }
            }
        }
    }

    pub fn join_game(&mut self) {
        self.send_request(Request::NewPlayer {
            name: self.player_name.clone(),
        })
        .unwrap();
        let response = self.read_response().unwrap();
        self.handle_response(&response);
    }

    pub fn play(&mut self) {
        'game: loop {
            let response = self.read_response();
            match response {
                Ok(response) => {
                    self.handle_response(&response);

                    match response {
                        Response::PlayerWin { board: _ }
                        | Response::PlayerLost { board: _ }
                        | Response::Draw { board: _ }
                        | Response::OtherPlayerDisconnected => break 'game,
                        _ => {}
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        println!("A conexão com o servidor foi perdida");
                        break 'game;
                    }
                }
            }
        }
    }

    fn handle_response(&mut self, response: &Response) {
        match response {
            Response::CreatedRoom => {
                println!("Sala criada. Aguardando outro jogador.");
                println!();
                println!("┼───────────────────────────────────────────────┼");
                println!("│            Você é o criador da sala,          │");
                println!("│       por favor, aguarde um jogador           │");
                println!("│           para começar a partida              │");
                println!("┼───────────────────────────────────────────────┼");
                
                println!();
                println!("═════════════════════════════════════════════════");
                println!("              Aguardando jogador...              ");
                println!("═════════════════════════════════════════════════");
                
                println!();

                // Aguardando jogador 2
                let response = self.read_response().unwrap();
                self.handle_response(&response);
            }
            Response::JoinedRoom { player_tile } => {
                println!(
                    "Entrou na sala. Você é o jogador {}.",
                    player_tile.to_number()
                );
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

    fn read_response(&mut self) -> Result<Response> {
        utils::read(&mut self.stream)
    }
}
