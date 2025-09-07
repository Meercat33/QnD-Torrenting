use std::net::SocketAddr;

use crate::types::{self, Handshake};

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn do_handshake(peer: &types::Peer) -> Result<tokio::net::TcpStream, std::io::Error> {

    let handshake_req = Handshake::new(&peer.associated_torrent);

    let mut stream: TcpStream = TcpStream::connect(SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port)).await?;
    stream.write_all(&handshake_req.bytes()).await?;

    let mut response = Vec::<u8>::new();
    stream.read_to_end(&mut response).await?;

    println!("{}", String::from_utf8_lossy(&response));

    Ok(stream)
 }