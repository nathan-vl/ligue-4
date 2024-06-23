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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut server = Server::new();
        server.listen();
    } else {
        let client = Client::new("127.0.0.1");
        match client {
            Ok(mut client) => {
                client.join_game();
                client.play();
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    println!("Não foi possível se conectar ao servidor.")
                }
            }
        }
    }
}
