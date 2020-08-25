use crate::mac::MacAddress;
use std::net::{SocketAddr, UdpSocket};

pub const WOL_HEADER: [u8; 6] = [255; 6];
pub const WOL_LENGTH: usize = 6 + 6 * 16;

pub struct WolPacket {
    pub bytes: [u8; WOL_LENGTH],
}

impl WolPacket {
    pub fn create(mac: &MacAddress) -> WolPacket {
        let mut packet = WolPacket {
            bytes: [0; WOL_LENGTH],
        };

        for (idx, header_byte) in WOL_HEADER.iter().enumerate() {
            packet.bytes[idx] = *header_byte;
            for rep in 1..17 {
                packet.bytes[idx + 6 * rep] = mac.bytes[idx];
            }
        }

        packet
    }
}

pub fn create_socket(tgt: &str) -> Result<UdpSocket, String> {
    let src_addrs = [
        SocketAddr::from(([0, 0, 0, 0], 9100)),
        SocketAddr::from(([0, 0, 0, 0], 9101)),
        SocketAddr::from(([0, 0, 0, 0], 9102)),
    ];

    let socket = match UdpSocket::bind(&src_addrs[..]) {
        Ok(socket) => socket,
        Err(_) => {
            return Err(
                "could not bind to address, check if ports 9100-9102 are in use".to_string(),
            )
        }
    };

    if let Err(_e) = socket.set_broadcast(true) {
        return Err("could not set broadcast on socket".to_string());
    }

    if let Err(_e) = socket.connect(tgt) {
        return Err(format!("could not connect to {}", tgt));
    }

    Ok(socket)
}
