use std::net::SocketAddr;
use std::time::Duration;

use crate::types::{self, Handshake};

use std::net::TcpStream;
use std::io::{Read, Write};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// pub async fn do_handshake(peer: &types::Peer) -> Result<tokio::net::TcpStream, std::io::Error> {

//     let handshake_req = Handshake::new(&peer.associated_torrent);

//     let mut stream: TcpStream = TcpStream::connect(SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port)).await?;
//     stream.write_all(&handshake_req.bytes()).await?;

//     let mut response = Vec::<u8>::new();
//     stream.read_to_end(&mut response).await?;

//     println!("{}", String::from_utf8_lossy(&response));

//     Ok(stream)
//  }

pub fn do_handshake(peer: &types::Peer) -> Result<std::net::TcpStream, std::io::Error> {

    let handshake_req = Handshake::new(&peer.associated_torrent);

    let mut stream: std::net::TcpStream = TcpStream::connect_timeout(
        SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port),
        Duration::from_secs(3)
    )?;

    stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set read timeout");
    stream.write_all(&handshake_req.bytes())?;

    let mut pstrlen_buf = [0u8; 20];
    let bytes_read: Result<Option<usize>, std::io::Error> = match stream.read(&mut pstrlen_buf) {
        Ok(0) => {
            eprintln!("Peer closed the connection.");
            Ok(None)
        },
        Ok(n) => {
            eprintln!("Got {} bytes", n);
            Ok(Some(n))
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
            eprintln!("Connection timed out");
            Ok(None)
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
            eprintln!("Connection reset by peer");
            Ok(None)
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
            eprintln!("Connection refused by peer");
            Ok(None)
        }   
        Err(e) => {
            eprintln!("Read Error");
            Err(e)
        }
    };
    match bytes_read {
        Ok(Some(n)) => println!("{} bytes read", n),
        Ok(None) => eprintln!("No bytes read, possibly due to a timeout, closed connection, or reset connection"),
        Err(e) => eprintln!("Something went wrong: {}", e)
    };

    // process_handshake_response(&mut response);

    Ok(stream)
 }

 pub fn process_handshake_response(res: &mut Vec<u8>) -> () {
    let proto = String::from_utf8_lossy(res.drain(..19).collect::<Vec<u8>>().as_slice()).into_owned();
    println!("{}", proto)
 }