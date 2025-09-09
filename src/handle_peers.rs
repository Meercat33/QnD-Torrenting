use std::net::SocketAddr;
use std::time::Duration;

use crate::types::{self, Handshake};

use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub async fn initialize_peer_connection(peer: &types::Peer) -> Result<TcpStream, std::io::Error> {

    let handshake_req = Handshake::new(&peer.associated_torrent);

    let stream  = TcpStream::connect(
        SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port)
    );

    let mut stream = timeout(Duration::from_secs(3), stream).await??;

    stream.write_all(&handshake_req.bytes()).await?;

    let mut response_buf = vec![0u8; 68];
    let bytes_read = Some(stream.read(&mut response_buf).await?);

    process_handshake_response(&mut response_buf);

    let mut bitfield_buf: Vec<u8> = vec![0u8; 4096];
    stream.read(&mut bitfield_buf).await?;
    Ok(stream)
 }

pub fn process_handshake_response(res: &[u8]) -> () {
    let proto = String::from_utf8_lossy(&res[..20]).into_owned();
    let reserve_bits = hex::encode(&res[20..28]);
    let info_hash = hex::encode(&res[28..48]);
    let peer_id = String::from_utf8_lossy(&res[48..68]).into_owned();
//     println!("PROTOCOL\t{}\n\
//             RESERVE BYTES\t{}\n\
//             INFO HASH\t{}\n\
//             PEER ID\t\t{}", 
//     proto, reserve_bits, info_hash, peer_id
// );
    eprintln!("{peer_id}");
 }

 pub fn process_bitfield_response() { // CHECK WIRESHARK BYTES!!! THEY KNOW!!!
    todo!();
 }