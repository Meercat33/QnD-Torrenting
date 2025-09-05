use std::{net::{IpAddr, Ipv4Addr, SocketAddr}};

pub struct Handshake {
    pub length: u8,
    pub protocol: Vec<u8>,
    pub reserved: Vec<u8>,
    pub info_hash: Vec<u8>,
    pub peer_id: Vec<u8>,
}

impl Handshake {
    pub fn new(info_hash: &[u8; 20]) -> Self {
        Self {
            length: 19,
            protocol: b"BitTorrent protocol".to_vec(),
            reserved: vec![0; 8],
            info_hash: info_hash.to_vec(),
            peer_id: b"nicknicknicknicknick".to_vec()
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(68);

        bytes.push(self.length);
        bytes.extend(self.protocol.clone());
        bytes.extend(self.reserved.clone());
        bytes.extend(self.info_hash.clone());
        bytes.extend(self.peer_id.clone());
        bytes
    }
}

pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Peer {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self {
            ip: ip,
            port: port,
        }
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}