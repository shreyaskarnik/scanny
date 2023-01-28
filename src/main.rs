use indicatif::ProgressBar;
use rayon::prelude::*;
use std::net::{IpAddr, TcpStream};
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
    TcpStream::connect((addr, port)).is_ok()
}

fn main() {
    let args = Cli::from_args();
    let host_ip = parse_address(&args.ip);
    println!("scanning {}", host_ip);
    let ports = (1..65535).collect::<Vec<u16>>();
    let pb = ProgressBar::new(ports.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .expect("failed to set progress bar style")
            .progress_chars("#>-"),
    );
    let results = ports
        .into_par_iter()
        .map(|port| {
            pb.inc(1);
            (port, scan(host_ip, port))
        })
        .filter(|&(_, open)| open)
        .collect::<Vec<(u16, bool)>>();
    pb.finish();
    for (port, _) in results {
        println!("open port {:?}", port);
    }
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
