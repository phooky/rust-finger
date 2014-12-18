// Finger server in Rust

extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;
use std::io::net::ip::{SocketAddr,IpAddr};
use std::io::net::tcp::{TcpListener,TcpStream};
use std::io::{Acceptor,Listener,BufferedStream,BufferedReader};
use std::io::fs::File;
use std::path::posix::Path;

//use std::time::duration::Duration;
//use std::io::net::addrinfo;
//use std::io::IoError;

static PORT_NUM : u16 = 79;
static LOCAL_IP : &'static str = "127.0.0.1";

mod passwd;

struct FingerdConfig {
    local_ip : String,
    permit_redirects : bool,
}

fn parse_config(path : &str) -> FingerdConfig {
    let mut config = FingerdConfig { local_ip : String::from_str(LOCAL_IP),
                                    permit_redirects : false };
    let p = Path::new(path);
    match File::open(&p) {
        Ok(f) => {
            let mut br = BufferedReader::new(f);
            for line in br.lines() {
                let st = line.unwrap();
                let s = st.as_slice().trim();
                if s.len() == 0 || s.starts_with("#") {
                    continue;
                }
                let kv : Vec<&str> = s.split('=').map(|x| x.trim()).collect();
                if kv.len() < 2 {
                    panic!("Unparseable line {}",s);
                }
                println!("Key {} Value {}",kv[0],kv[1]);
            }
        }
        Err(..) => (),
    }
    config
}

//Login: phooky         			Name: Adam Mayer
//Directory: /home/phooky             	Shell: /bin/bash
//On since Thu Dec  4 15:46 (EST) on tty2     2 days 23 hours idle
//     (messages off)
//On since Sat Nov 29 14:01 (EST) on :0 from :0 (messages off)
//On since Tue Dec  2 14:28 (EST) on pts/3 from :0
//    5 days 1 hour idle
//On since Thu Dec  4 16:05 (EST) on pts/1 from :0
//    45 seconds idle
//No mail.
//No Plan.
fn format_verbose(e : & passwd::Entry, bstream : &mut BufferedStream<TcpStream> ) {
    writeln!(bstream,"Login: {: <33}Name: {: <33}",
             e.username,e.gecos.name);
    if e.gecos.location.len() > 0 || 
        e.gecos.telephone.len() > 0 {
        writeln!(bstream,"Office: {: <30}Telephone: {: <30}",
                 e.gecos.location, e.gecos.telephone);
    }
    writeln!(bstream,"Directory: {: <29}Shell: {: <33}",
             e.home,e.shell);
}

fn process_local(uname : &str, wflag : bool, bstream : &mut BufferedStream<TcpStream>) {
    match passwd::get_entry(String::from_str(uname)) {
        None => {writeln!(bstream,"{}: no such user.",uname);}
        Some(e) => {format_verbose(&e,bstream);}
    };
}

fn finger_remote(req:&str, remote_host:&str, wflag:bool, bstream : &mut BufferedStream<TcpStream>) {
    println!("Remote! {} at {}, {}",req,remote_host,wflag);
}

fn process_request(str : String, bstream : &mut BufferedStream<TcpStream>) {
    let mut s = str.as_slice().trim();
    let wflag = s.starts_with("/W");
    if wflag {
        s = s.slice_from(2).trim();
    }
    match s.rfind('@') {
        None => process_local(s,wflag,bstream),
        Some(x) => finger_remote(s.slice_to(x),s.slice_from(x+1),wflag,bstream),
    }
}

fn finger_client(mut stream: TcpStream) {
//    stream.set_read_timeout(None);
    let mut bstream : BufferedStream<TcpStream> = BufferedStream::new(stream);
    match bstream.read_line() {
        Ok(str) => process_request(str,&mut bstream),
        Err(e) => panic!("Read failed: {}",e.desc),
    }
}

fn main() {
    let opts = &[
        optopt("","ip","IP to host server on","IP_ADDR"),
        optopt("c","","Config file (defaults to /etc/fingerd.conf)",
               "CONFIG_PATH"),
        optflag("h","help","Show usage information"),
    ];
    let args : Vec<String> = os::args();
    let program = args[0].clone();
    let matches = match getopts(args.tail(),opts) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    if matches.opt_present("h") {
        println!("{}",usage("Finger protocol server",opts));
        return;
    }
    let config = match matches.opt_str("c") {
        Some(path) => parse_config(path.as_slice()),
        None => parse_config("/etc/fingerd.conf"),
    };
    let ip_addr : IpAddr;
    match from_str::<IpAddr>(config.local_ip.as_slice()) {
        None => panic!("Can not parse local IP specification!"),
        Some(ip) => ip_addr = ip,
    }
    let mut tcp_listener =
        match TcpListener::bind(SocketAddr{ ip:ip_addr, port:PORT_NUM }) {
            Ok(l) => l,
            Err(..) => panic!("Cannot bind to port {}!",PORT_NUM),
        };
    let mut acceptor = 
        match tcp_listener.listen() {
            Ok(a) => a,
            Err(..) => panic!("Can not listen on port {}!",PORT_NUM),
        };
    println!("Starting to listen on port {}...",PORT_NUM);
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => println!("Incoming connection failed."),
            Ok(stream) => spawn( proc() { finger_client(stream) } ),
        }
    }
}
