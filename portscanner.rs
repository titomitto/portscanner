use std::env;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Instant;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid amount of arguments");
        println!("Syntax: terrible_scanner <ip_address>");
        process::exit(1);
    }

    let target = args[1].clone();
    let addrs: Vec<SocketAddr> = match (target.as_str(), 0).to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => {
            println!("Cannot resolve hostname");
            process::exit(1);
        }
    };

    let target_ip = addrs[0].ip();

    let lines = "-".repeat(50);

    println!("{}", lines);
    println!("Scanning target {}", target_ip);
    println!("Start time: {:?}", Instant::now());
    println!("{}", lines);

    for port in 1..=1000 {
        let addr = format!("{}:{}", target_ip, port);
        match TcpStream::connect_timeout(&addr.parse().unwrap(), std::time::Duration::from_secs(1)) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => (),
        }
    }
}