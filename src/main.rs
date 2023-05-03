use std::net::IpAddr;
use std::str::FromStr;
use clap::{App, Arg};
use tokio::net::TcpStream;
use std::net::{ToSocketAddrs, SocketAddr};
use std::time::Duration;
use tokio::time::timeout;

async fn scan_port(target_ip: IpAddr, port: u16) {
    let addr = format!("{}:{}", target_ip, port);
    let stream =  TcpStream::connect(addr).await;

    if let Ok(_) = stream {
        println!("Port {} is open", port);
    }
}

fn get_ip_address(domain: &str) -> Result<Vec<SocketAddr>, std::io::Error> {
    let addr = format!("{}:80", domain);
    let socket_addrs = addr.to_socket_addrs()?.collect();
    Ok(socket_addrs)
}

#[tokio::main]
async fn main() {
    let matches = App::new("PortScanner")
            .arg(Arg::with_name("DOMAIN")
            .help("The domain or IP address to scan")
            .required(true)
            .index(1))
        .arg(Arg::with_name("MAX_PORT")
            .help("The maximum port number to scan")
            .takes_value(true)
            .short('m')
            .long("max-port")
            .default_value("1000"))
        .get_matches();

        let domain_or_ip = matches.value_of("DOMAIN").unwrap();
        let target_ip = match IpAddr::from_str(domain_or_ip) {
            Ok(ip) => ip,
            Err(_) => {
                let ips = get_ip_address(domain_or_ip).unwrap();
                if let Some(ip) = ips.first() {
                    ip.ip()
                } else {
                    eprintln!("Error: Unable to resolve domain name to an IP address.");
                    return;
                }
            }
        };
    

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