use std::net::{IpAddr, TcpStream};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The ip to scan
    #[structopt(
        long = "target-ip",
        help = "ip of the target",
        default_value = "127.0.0.1"
    )]
    ip: String,
}

fn parse_address(address: &str) -> IpAddr {
    return address.parse::<IpAddr>().expect("not a valid ip address");
}
fn scan(addr: IpAddr, port: u16) {
    match TcpStream::connect((addr, port)) {
        Ok(_) => {
            // Found open port, indicate progress and send to main thread
            println!("port {} open", port);
        }
        Err(_) => {
            //  println!("{} closed", port);
        }
    }
}

fn main() {
    let args = Cli::from_args();
    let host_ip = parse_address(&args.ip);
    println!("{}", host_ip);
    for x in 0..65535 {
        scan(host_ip, x)
    }
}
