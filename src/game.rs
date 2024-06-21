use std::io::{stdin, stdout, Write};

use crate::{board::Board, tile::Tile};

pub struct Game {
    pub board: Board,
    current_player: Tile,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Tile::Player1,
        }
    }
}

fn play() {
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
            if board.check_win(&current_player, dest.0, dest.1) {
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
