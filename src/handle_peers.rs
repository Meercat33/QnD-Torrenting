use std::any::{type_name, Any};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex, LazyLock};
use std::time::Duration;

use crate::types::{self, Handshake, Message};

use lava_torrent::torrent;
use lava_torrent::torrent::v1::Torrent;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const TIMEOUT_DURATION: Duration = Duration::from_secs(3);

pub async fn do_peer_connection(peer: &types::Peer, torrent: Torrent, peer_map: types::LockedPeerMap) -> Result<TcpStream, std::io::Error> {
    let piece_hashes = &torrent.pieces;
    let num_pieces = &torrent.pieces.len();
    let piece_length = &torrent.piece_length;

    let handshake_req = Handshake::new(&peer.associated_torrent);

    let stream  = TcpStream::connect(
        SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port)
    );

    let mut stream = timeout(TIMEOUT_DURATION, stream).await??;

    stream.write_all(&handshake_req.bytes()).await?;

    let mut response_buf = vec![0u8; 68];
    stream.read(&mut response_buf).await?;

    eprint!("{}:{}\t", peer.ip, peer.port);

    process_handshake_response(&mut response_buf);

    let mut next_msg_buf: Vec<u8> = vec![0u8; 4096];
    stream.read(&mut next_msg_buf).await?;
    while next_msg_buf.len() > 0 {
        let mut length: usize = 0;
        for x in &next_msg_buf[..4] { length += *x as usize; }

        let message = parse_message(&next_msg_buf[4..length + 4]);
        match message {
            Message::Bitfield { payload } => println!("{:?}", payload),
            _ => break
        }

        break;
    }


    Ok(stream)
}

pub fn process_handshake_response(res: &[u8]) -> () { // TODO: Create functionality to check if the handshake is valid
    let peer_id = String::from_utf8_lossy(&res[48..68]).into_owned();
    println!("{peer_id}");
 }

// Finds and returns the type of TCP/BitTorrent message sent by the peer after their handshake.
// Takes a Vec<u8> of raw bytes read from a TCP stream and returns an enum Message
fn parse_message(msg: &[u8]) -> Message {
    let msg_id = msg[0];
    match msg_id {
        0 => Message::Choke,
        1 => Message::Unchoke,
        2 => Message::Interested,
        3 => Message::NotInterested,
        4 => Message::Have,
        5 => Message::Bitfield {payload: msg[1..].to_vec()},
        6 => Message::Request,
        7 => Message::Piece,
        8 => Message::Cancel,
        9 => Message::Port,
        _ => Message::InvalidMessage
    }
}

fn handle_bitfield(peer: types::Peer, num_pieces: usize, bitfield: Vec<u8>, map: types::LockedPeerMap) {
    let ip = peer.ip;
    
}
