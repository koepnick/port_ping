extern crate clap;
extern crate colored;

use clap::{Arg, App};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use colored::*;

fn check_port(host: &str, port: u16) -> bool {
    let socket_addr = (host, port);
    match socket_addr.to_socket_addrs() {
        Ok(mut addrs) => {
            while let Some(addr) = addrs.next() {
                match TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
                    Ok(_) => return true,
                    Err(_) => (),
                }
            }
            false
        }
        Err(_) => false,
    }
}

fn main() {
    let matches = App::new("port_checker")
        .arg(Arg::with_name("host")
            .help("The host to check")
            .required(true)
            .index(1))
        .arg(Arg::with_name("port")
            .help("The port to check")
            .required(true)
            .index(2))
        .arg(Arg::with_name("count")
            .short("c")
            .long("count")
            .takes_value(true)
            .default_value("-1")
            .help("Stop after sending count connection requests."))
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .takes_value(true)
            .default_value("1")
            .help("Wait interval seconds between sending each packet"))
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
    let timeout = matches.value_of("timeout").unwrap().parse::<u64>().unwrap();

    let mut i = 0;
    while count < 0 || i < count {
        let open = check_port(host, port);
        if open {
            println!("Port {} on host {} is {}", port, host, "open".green());
        } else {
            println!("Port {} on host {} is {}", port, host, "closed".red());
        }
        i += 1;
        std::thread::sleep(Duration::from_secs(timeout));
    }
}

