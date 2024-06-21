mod board;
mod client;
mod game;
mod game_room;
mod request;
mod response;
mod server;
mod tile;
mod utils;

use std::env;

use client::Client;
use server::Server;

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
        client.play();
    }
}
