use std::{
    io::{BufRead, BufReader, Result, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

pub fn read<T: for<'a> Deserialize<'a>>(stream: &mut TcpStream) -> Result<T> {
    let mut buf_reader = BufReader::new(stream);

    let mut buf = vec![];
    buf_reader.read_until(b'\n', &mut buf)?;
    let buf = String::from_utf8_lossy(&buf);
    let value = serde_json::from_str(&buf)?;

    Ok(value)
}

pub fn send<T: Serialize>(stream: &mut TcpStream, message: &T) -> Result<()> {
    let message_str = serde_json::to_string(message)? + "\n";
    stream.write_all(message_str.as_bytes())?;

    Ok(())
}
