use getopt::Opt;

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

    let node_plus_port = [network_name, port].join(":");
    println!("{}", node_plus_port);
}
