mod board;
mod tile;

use std::io::{stdin, stdout, Write};

use board::Board;
use tile::Tile;

fn main() {
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

            if current_player == Tile::Player1 {
                current_player = Tile::Player2;
            } else {
                current_player = Tile::Player1;
            }
        }
    }
}
