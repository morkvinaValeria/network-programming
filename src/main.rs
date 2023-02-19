use getopt::Opt;
use std::net::{ToSocketAddrs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "a:b:v");

    let mut node = String::new();
    let mut port = String::new();
    let mut ip_version = String::new();

    loop {
        match opts.next().transpose() {
            Ok(None) => break,
            Ok(Some(opt)) => match opt {
                Opt('a', Some(arg)) => node = arg.clone(),
                Opt('b', Some(arg)) => port = arg.clone(),
                Opt('v', Some(arg)) => {
                    let clone = arg.clone();
                    ip_version = clone.trim();
                },
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
                if is_ipv6(sock_addr) && ip_version == "6"{
                    println!("\nSocket address:\n address: {s_ip}, port: {s_port}")
                }
                if is_ipv4(sock_addr) && ip_version == "4"{
                    println!("\nSocket address:\n address: {s_ip}, port: {s_port}")
                }
            }
        }
    }
}
