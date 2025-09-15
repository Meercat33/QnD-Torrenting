use std::{collections::HashMap, net::Ipv4Addr};
use std::sync::{Arc, Mutex};

pub type LockedPeerMap = Arc<Mutex<HashMap<Ipv4Addr, Vec<u8>>>>;
pub struct Handshake {
    pub length: u8,
    pub protocol: Vec<u8>,
    pub reserved: Vec<u8>,
    pub info_hash: Vec<u8>,
    pub peer_id: Vec<u8>,
}

impl Handshake {
    pub fn new(info_hash: &Vec<u8>) -> Self {
        Self {
            length: 19,
            protocol: b"BitTorrent protocol".to_vec(),
            reserved: vec![0; 8],
            info_hash: info_hash.to_vec(),
            peer_id: b"nickrust-28jd7l931ks".to_vec()
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
    pub associated_torrent: Vec<u8>,
    pub peer_id: String
}

impl Peer {
    pub fn new(ip: Ipv4Addr, port: u16, info_hash: Vec<u8>, peer_id: String) -> Self {
        Self {
            ip: ip,
            port: port,
            associated_torrent: info_hash,
            peer_id: peer_id
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


pub enum Message {
    Choke,
    Unchoke,
    Interested,
    Have,
    Bitfield {payload: Vec<u8>},
    Piece,
    Request,
    KeepAlive,
    NotInterested,
    Cancel,
    Port,
    InvalidMessage
} //TODO create structs and impl them for different 

