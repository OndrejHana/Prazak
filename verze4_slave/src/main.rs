use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpListener;

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Settings {
    port: u16,
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
        let mut master = stream.unwrap();
        let mut reader = BufReader::new(&master);
        let mut writer = BufWriter::new(&master);

        let mut bytes_read;
        let mut buf = vec![0u8; 32];
        loop {
            bytes_read = reader.read(&mut buf).unwrap();

            if bytes_read != 0 {
                break;
            }
        }

        let str = std::str::from_utf8(&buf[..bytes_read]).unwrap();
        if str.contains("GET_COMPUTES") {
            let buf = vec![num_cpus::get() as u8];
            writer.write(&buf).unwrap();
        }
        println!("Hello, world!");
    }
}
