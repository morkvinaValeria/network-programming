use getopt::Opt;
use std::net::{SocketAddr, ToSocketAddrs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "a:b:");

    let mut node = String::new();
    let mut port = String::new();

    loop {
        match opts.next().transpose() {
            Ok(None) => break,
            Ok(Some(opt)) => match opt {
                Opt('a', Some(arg)) => node = arg.clone(),
                Opt('b', Some(arg)) => port = arg.clone(),
                _ => unreachable!(),
            },
            Err(_) => println!("Input is invalid")
        }
    }

    let node_plus_port = [node, port].join(":");
    println!("{}", node_plus_port);

    let mut addrs_iter = node_plus_port.to_socket_addrs().unwrap();
    loop{
        match addrs_iter.next().transpose() {
            Ok(None) => break,
            Ok(Some(sock_addr)) => {
                let  = sock_addr.ip().to_string();
                let s_port = sock_addr.port().to_string();
                println!("Socket address:\n address: {s_ip}, port: {s_port}")
            }
            Err(_) => println!("Input is invalid")
        }
    }
}
