// Finger server in Rust

use std::io::net::ip::{SocketAddr,IpAddr};
use std::io::net::tcp::{TcpListener,TcpStream};
use std::io::{Acceptor,Listener,BufferedReader};
//use std::time::duration::Duration;
//use std::io::net::addrinfo;
//use std::io::IoError;

static PORT_NUM : u16 = 79;
static LOCAL_IP : &'static str = "127.0.0.1";

fn process_request(str : String) {
    println!("Process request {}.",str);
}

fn finger_client(mut stream: TcpStream) {
//    stream.set_read_timeout(None);
    let mut reader = BufferedReader::new(stream);
    match reader.read_line() {
        Ok(str) => process_request(str),
        Err(e) => panic!("Read failed: {}",e.desc),
    }
}

fn main() {
    let ip_addr : IpAddr;
    match from_str::<IpAddr>(LOCAL_IP) {
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
