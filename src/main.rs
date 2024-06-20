mod board;
mod game_room;
mod tile;
mod server;
mod request;
mod client;
mod response;

use std::{
    env, io::{stdin, stdout, Write}, net::TcpListener
};

use board::Board;
use client::Client;
use server::{Server, PORT};
use tile::Tile;

fn game() {
    let mut board = Board::new();

    let mut current_player = Tile::Player1;
    loop {
        board.print();
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

        println!();

        if let Some(dest) = board.place_tile(col as usize, &current_player) {
            if board.check_column(&current_player, dest.0)
                || board.check_row(&current_player, dest.1)
                || board.check_direct_diagonal(&current_player)
                || board.check_inverse_diagonal(&current_player)
            {
                println!(
                    "O jogador {} ganhou. Resultado:",
                    if current_player == Tile::Player1 {
                        1
                    } else {
                        2
                    }
                );
                board.print();
                break;
            }

            current_player = current_player.opposite();
        }
    }
}

/*
- Vazia: lista de salas ou criar sala
- Entrar em sala
- Jogada da partida
- Rematch
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut server = Server::new();
        server.listen();
    } else {
        let mut client = Client::new("127.0.0.1");
        client.join_game();
    }
}
