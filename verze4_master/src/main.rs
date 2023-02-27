mod approx_graph;

use std::io::prelude::*;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc;
use std::thread;

use config::Config;
use serde::{Deserialize, Serialize};

use graph::{Graph, GraphFs};

#[derive(Serialize, Deserialize, Debug)]
struct Slave {
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    graph_source: String,
    port: u16,
    slaves: Vec<Slave>,
}

fn handle_slave() {}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("./master_config"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    // connect to slave apps

    let mut compute_count = 0;
    let mut handles = Vec::new();

    let (tx, rx) = mpsc::channel();

    for slave in config.slaves {
        let mut slave_stream = TcpStream::connect(format!("{}:{}", slave.ip, slave.port)).unwrap();

        slave_stream.write(b"GET_COMPUTES").unwrap();

        let mut buf = vec![0u8];
        slave_stream.read(&mut buf).unwrap();

        compute_count += buf[0] as usize;

        handles.push(thread::spawn(move || {
            handle_slave();
        }));
    }

    let mut graphs = GraphFs::new();
    graphs.load_from_file(&config.graph_source).unwrap();

    for graph in graphs {
        assert!(
            graph.matrix.len() % compute_count == 0,
            "The number of vertices has to be divisable by the number of cores of all slaves!"
        );

        let parts = approx_graph::partition_graph(&graph, compute_count);
        for partition in parts.chunks(handles.len()) {}
    }
}
