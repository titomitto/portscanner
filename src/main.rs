use std::net::IpAddr;
use std::str::FromStr;
use clap::{App, Arg};
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::timeout;

async fn scan_port(target_ip: IpAddr, port: u16) {
    let addr = format!("{}:{}", target_ip, port);
    let stream =  TcpStream::connect(addr).await;

    if let Ok(_) = stream {
        println!("Port {} is open", port);
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("PortScanner")
        .arg(Arg::with_name("IP_ADDRESS")
            .help("The IP address to scan")
            .required(true)
            .index(1))
        .arg(Arg::with_name("MAX_PORT")
            .help("The maximum port number to scan")
            .takes_value(true)
            .short('m')
            .long("max-port")
            .default_value("1000"))
        .get_matches();

    let target_ip = matches.value_of("IP_ADDRESS").unwrap();
    let target_ip = IpAddr::from_str(target_ip).unwrap();

    let max_port = matches.value_of("MAX_PORT").unwrap().parse::<u16>().unwrap();

    let lines = "-".repeat(50);
    let start_time = chrono::Local::now();

    println!("{}", lines);
    println!("Scanning target {} up to port {}", target_ip, max_port);
    println!("Start time: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
    println!("{}", lines);

    let mut tasks = Vec::new();
    for port in 1..=max_port {
        let target_ip = target_ip.clone();
        let task = tokio::spawn(async move {
            timeout(Duration::from_secs(1), scan_port(target_ip, port)).await
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
    println!("Scanning complete!");
}