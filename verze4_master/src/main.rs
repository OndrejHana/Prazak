use std::io::{prelude::*, BufReader, BufWriter};
use std::net::IpAddr;

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Slave {
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    port: u16,
    slaves: Vec<Slave>,
}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("./master_config"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let mut slaves = Vec::new();
    for slave in config.slaves {
        slaves.push(
            std::net::TcpStream::connect(format!("{}:{}", slave.ip, slave.port))
                .expect("could not connect to configured slave"),
        );
    }

    let mut computes = 0;

    for slave in slaves.iter() {
        let mut reader = BufReader::new(slave.clone());
        let mut writer = BufWriter::new(slave);

        writer.write_all(b"GET_COMPUTES\n").unwrap();
        println!("sent!");

        let mut buf = vec![0];
        let mut bytes_read = 0;
        while bytes_read == 0 {
            bytes_read = reader.read_to_end(&mut buf).unwrap();
        }
        println!("{buf:?}");
    }

    if computes == 0 {
        panic!();
    }

    println!("{computes}");
}
