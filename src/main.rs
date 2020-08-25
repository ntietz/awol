use awol::mac::MacAddress;
use awol::wol::{create_socket, WolPacket};
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&3) {
        Ordering::Greater => {
            println!("awol: too many arguments");
            usage_quit();
        }
        Ordering::Less => {
            println!("awol: too few arguments");
            usage_quit();
        }
        Ordering::Equal => {}
    }

    let mac_addr = match MacAddress::parse(&args[1]) {
        Some(mac_addr) => mac_addr,
        None => {
            println!("awol: could not parse MAC address");
            usage_quit();
        }
    };

    let tgt_addr = &args[2];

    let socket = match create_socket(&tgt_addr) {
        Ok(socket) => socket,
        Err(message) => {
            println!("awol: {}", message);
            usage_quit();
        }
    };

    let packet = WolPacket::create(&mac_addr);

    let result = socket.send_to(&packet.bytes, &tgt_addr);

    match result {
        Ok(_) => println!("Successfully sent wake-up packet!"),
        Err(_) => {
            println!("Failed to send packet.");
            std::process::exit(1);
        }
    }
}

fn usage_quit() -> ! {
    println!("Usage: awol <mac-addr> <host:port>");
    std::process::exit(1);
}
