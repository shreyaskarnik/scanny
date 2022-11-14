use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::net::{IpAddr, TcpStream};
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The ip to scan
    #[structopt(long = "target-ip", help = "ip of the target")]
    ip: String,
}

fn parse_address(address: &str) -> IpAddr {
    return address.parse::<IpAddr>().expect("not a valid ip address");
}
fn scan(addr: IpAddr, port: u16) -> bool {
    match TcpStream::connect((addr, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    let start = Instant::now();
    let args = Cli::from_args();
    let host_ip = parse_address(&args.ip);
    println!("scanning {}", host_ip);
    let ports = (1..65535).into_par_iter();
    let open_ports: Vec<bool> = ports
        .progress()
        .map(|port| scan(host_ip, port as u16))
        .filter_map(|x| Some(x))
        .collect();
    for (i, x) in open_ports.iter().enumerate() {
        if *x {
            println!("open port {:?}", i + 1);
        }
    }
    let duration = start.elapsed();
    println!("scanned host in {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_address() {
        parse_address(&String::from("1.1.1.1"));
    }
    #[test]
    #[should_panic(expected = "not a valid ip address: AddrParseError(Ip)")]
    fn test_parse_address_fail() {
        parse_address(&String::from("1.1.1."));
    }
}
