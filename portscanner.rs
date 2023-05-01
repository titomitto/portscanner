use std::env;
use std::net::IpAddr;
use std::process;
use std::str::FromStr;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::time::Duration;

async fn scan_port(target_ip: IpAddr, port: u16) {
    let addr = format!("{}:{}", target_ip, port);
    let stream = TcpStream::connect(addr).await;

    if let Ok(_) = stream {
        println!("Port {} is open", port);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid amount of arguments");
        println!("Syntax: terrible_scanner <ip_address>");
        process::exit(1);
    }

    let target_ip = match IpAddr::from_str(&args[1]) {
        Ok(ip) => ip,
        Err(_) => {
            println!("Cannot resolve hostname");
            process::exit(1);
        }
    };

    
    let max_port = if args.len() >= 3 {
        match args[2].parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("Invalid max port number");
                process::exit(1);
            }
        }
    } else {
        1000
    };

    

    let lines = "-".repeat(50);

    println!("{}", lines);
    println!("Scanning target {}", target_ip);
    println!("Start time: {:?}", Instant::now());
    println!("{}", lines);

    let mut tasks = Vec::new();
    for port in 1..=max_port {
        let target_ip = target_ip.clone();
        let task = tokio::spawn(async move {
            let result = timeout(Duration::from_secs(1), scan_port(target_ip, port)).await;
            if result.is_err() {
                //println!("Port {} is closed", port);
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        let _ = task.await;
    }
}