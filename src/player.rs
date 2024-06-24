use std::net::TcpStream;

pub struct Player {
    pub name: String,
    pub stream: TcpStream,
}

impl Player {
    pub fn new(name: String, stream: TcpStream) -> Self {
        Self { name, stream }
    }
}
