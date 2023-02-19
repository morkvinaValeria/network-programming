use getopt::Opt;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "a:b:c:");

    let mut network_address = String::new();
    let mut network_name = String::new();
    let mut port = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', Some(arg)) => network_address = arg.clone(),
                Opt('b', Some(arg)) => network_name = arg.clone(),
                Opt('c', Some(arg)) => port = arg.clone(),
                _ => unreachable!(),
            },
        }
    }
    let network_address_plus_port = [network_address, port].join(":");
    let network_name_plus_port = [network_name, port].join(":");
    println!("{}", network_address_plus_port);
    println!("{}", network_name_plus_port);
}
