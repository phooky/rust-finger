// Finger client in Rust

use std::os;
use std::io::net::ip::{SocketAddr,IpAddr};
use std::io::net::tcp::TcpStream;
use std::time::duration::Duration;
use std::io::net::addrinfo;
use std::io::IoError;
use std::fmt;

static PORT_NUM: u16 = 79;

struct Options {
    short : bool,
    long : bool,
    no_plan : bool,
    no_matching : bool,
}

fn main() {
    let (opts, targets) = parse_arguments(os::args());
    for target in targets.iter() {
        finger(target,opts);
    }
}

fn finger(target : &String, opts : Options) -> Result<(), IoError> {
    let ts = (*target).as_slice();
    match ts.rfind('@') {
        None => finger_local(target,opts),
        Some(x) => finger_remote(ts.slice_to(x),ts.slice_from(x+1),opts),
    }
}

fn finger_local(target:&String,opts:Options) -> Result<(), IoError> {
    println!("local {}",target);
    Ok(())
}

fn finger_remote(req:&str,hostname:&str,opts:Options) -> Result<(), IoError> {
    println!("remote {} from host {}",req,hostname);
    let mut ips : Vec<IpAddr>;
    match addrinfo::get_host_addresses(hostname) {
        Err(e) => return Err(e),
        Ok(v) => ips = v,
    }
    ips.dedup();
    for ip in ips.iter() {
        println!("Trying {}...",ip);
        let addr = SocketAddr{ ip : *ip, port : PORT_NUM };
        match TcpStream::connect_timeout(addr,Duration::seconds(5)) {
            Ok(mut stream) => {
                stream.write(format!("{}\n\r",req).as_bytes());
            },
            Err(e) => match e.kind {
                std::io::IoErrorKind::TimedOut => continue,
                _ => panic!("Unexpected error on connect!"),
            }
        }
        println!("Finished {}...",ip);
    }
    Ok(())
}

fn parse_arguments(arguments : Vec<String>) -> (Options, Vec<String>) {
    let mut targets: Vec<String> = Vec::new();
    let mut opts = Options {
        short : false,
        long : false,
        no_plan : false,
        no_matching : false,
    };

    let mut argiter = arguments.iter();
    argiter.next(); // skip first argument (app name)
    for arg in argiter {
        if arg.as_slice().starts_with("-") {
            // iterate over flags
            let mut it = arg.as_slice().graphemes(true);
            it.next(); // discard initial flag
            for g in it {
                match g {
                    "l" => opts.long = true,
                    "s" => opts.short = true,
                    "p" => opts.no_plan = true,
                    "m" => opts.no_matching = true,
                    _ => panic!("Unrecognized flag!"),
                }
            }
        } else {
            targets.push(String::from_str(arg.as_slice()))
        }
    }
    (opts, targets)
}
