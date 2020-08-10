pub struct MacAddress {
    pub bytes: [u8; 6],
}

impl MacAddress {
    pub fn parse(mac: &str) -> Option<MacAddress> {
        // TODO: return a Result instead of Option so we can be specific about the failure
        let mac = mac.replace(&[':', '-', '.'][..], "");
        let mut mac_addr: [u8; 6] = [0; 6];

        if mac.len() != 12 {
            return None;
        }

        for idx in 0..6 {
            mac_addr[idx] = match u8::from_str_radix(&mac[2 * idx..2 * idx + 2], 16) {
                Ok(x) => x,
                Err(_) => return None,
            }
        }

        return Some(MacAddress { bytes: mac_addr });
    }
}
