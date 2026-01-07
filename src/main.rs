use std::env;
use std::net::TcpStream;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("usage: peer-latency-check <host:port> [host:port]");
        return;
    }

    for peer in args {
        check_peer(&peer);
    }
}

fn check_peer(addr: &str) {
    let start = Instant::now();

    let result = TcpStream::connect_timeout(
        &addr.parse().expect("invalid address"),
        Duration::from_secs(3),
    );

    match result {
        Ok(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("{}: {} ms", addr, elapsed);
        }
        Err(e) => {
            println!("{}: connection failed ({})", addr, e);
        }
    }
}