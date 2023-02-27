use std::io::{prelude::*, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Settings {
    port: u16,
}

fn init(stream: &mut TcpStream) {
    let mut buf = vec![0u8; 128];

    let bytes_read = stream.read(&mut buf).unwrap();
    let msg = std::str::from_utf8(&buf[0..bytes_read]).unwrap();

    if msg.contains("GET_COMPUTES") {
        let cores = num_cpus::get() as u8;
        stream.write(&[cores]).unwrap();
    }
}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("./slave_config"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        init(&mut stream);
    }
}
