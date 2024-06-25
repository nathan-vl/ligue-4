mod board;
mod client;
mod game;
mod game_room;
mod player;
mod request;
mod response;
mod server;
mod tile;
mod utils;

use std::{
    env,
    io::{stdin, stdout, Write},
};

use client::Client;
use server::Server;

fn read_line(question: String) -> String {
    print!("{}", question);
    _ = stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_input_range_inclusive<T>(range_start: T, range_end: T) -> T
where
    T: std::str::FromStr + PartialOrd<T> + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        print!("> ");
        _ = stdout().flush();

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        if let Ok(value) = s.trim().parse() {
            if range_start <= value && value <= range_end {
                return value;
            }
        }

        println!("Valor deve ser de {} e {}", range_start, range_end);
    }
}

fn no_args_init() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║            Bem Vindo ao Ligue 4!             ║");
    println!("╚══════════════════════════════════════════════╝");
    
    println!("                ╔═════════════╗");
    println!("                ║X O X O X O X║");
    println!("                ║O X O X O X O║");
    println!("                ║X O X O X O X║");
    println!("                ║O X O X O X O║");
    println!("                ║X O X O X O X║");
    println!("                ║O X O X O X O║");
    println!("                ╚═════════════╝");
    println!();
    println!("Pressione: ");
    println!("[1] - para Jogar");
    println!("[2] - para Iniciar um Servidor ");

    let option = read_input_range_inclusive(1, 2);
    if option == 1 {
        let name = read_line("Informe o seu nome: ".to_owned());
        let ip = read_line("Informe o IP do servidor: ".to_owned());

        let mut client = Client::new(name, &ip);
        client.play();
    } else {
        let mut server = Server::new();
        server.listen();
    }
}

fn main() {
    const USAGE: &str = "uso:
\t./ligue-4
\tOU
\t./ligue-4 server
\tOU
\t./ligue-4 client <nome> <server ip>";

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        no_args_init();
    } else if args.len() == 2 && args[1] == "server" {
        let mut server = Server::new();
        server.listen();
    } else if args.len() == 4 && args[1] == "client" {
        let player_name = &args[2];
        let server_ip = &args[3];
        let mut client = Client::new(player_name.to_string(), server_ip);
        client.play();
    } else {
        println!("{USAGE}");
    }
}
