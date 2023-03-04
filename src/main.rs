use getopt::Opt;
use std::net::{ToSocketAddrs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "a:b:v:");

    let mut node = String::new();
    let mut port = String::new();
    let mut ip_version = "0";

    loop {
        match opts.next().transpose() {
            Ok(None) => break,
            Ok(Some(opt)) => match opt {
                Opt('a', Some(arg)) => node = arg.clone(),
                Opt('b', Some(arg)) => port = arg.clone(),
                Opt('v', Some(arg)) => ip_version = arg.clone(),
                _ => unreachable!(),
            },
            Err(_) => println!("Input is invalid")
        }
    }

    let node_plus_port = [node, port].join(":");
    println!("{}", node_plus_port);

    let mut addrs_iter = node_plus_port.to_socket_addrs().unwrap();
    loop{
        match addrs_iter.next() {
            None => break,
            Some(sock_addr) => {
                let s_ip = sock_addr.ip().to_string();
                let s_port = sock_addr.port().to_string();
                let ip_version_trim = ip_version.trim();
                if (ip_version_trim == "0"){
                    println!("\nSocket addresses:\n address: {s_ip}, port: {s_port}")
                } else if sock_addr.is_ipv6() && ip_version_trim == "6"{
                    println!("\nSocket IpV6 addresses:\n address: {s_ip}, port: {s_port}")
                }else if sock_addr.is_ipv4() && ip_version_trim == "4"{
                    println!("\nSocket IpV4 addresses:\n address: {s_ip}, port: {s_port}")
                }
            }
        }
    }
}
