use getopt::Opt;

fn main() {
    // args = ["network_address", "network_name", "port"];
    let mut opts = getopt::Parser::new(&args, "a:b:c:");

    let mut a_flag = String::new();
    let mut b_flag = String::new();
    let mut c_flag = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', Some(arg)) => a_flag = arg.clone(),
                Opt('b', Some(arg)) => b_flag = arg.clone(),
                Opt('c', Some(arg)) => c_flag = arg.clone(),
                _ => unreachable!(),
            },
        }
    }

    let new_args = args.split_off(opts.index());


    println!("{new_args}");
}
