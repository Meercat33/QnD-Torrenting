use crate::errors;
use crate::types::Peer;

use lava_torrent::torrent::v1::Torrent;
use std::{io::{Write}, net::{self, TcpStream, Ipv4Addr}};
use url::Url;
use errors::BTError;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use native_tls::{TlsConnector, TlsStream};
use bip_bencode::{BRefAccess, BencodeRef};
use hex::decode;

pub fn split_http_from_bencoded_bytes(resp: &[u8]) -> Option<(&[u8], &[u8])> {
    let sep = b"\r\n\r\n";
    if let Some(pos) = resp.windows(sep.len()).position(|w| w == sep) {
        let headers = &resp[..pos];
        let bytes = &resp[pos + sep.len()..];
        Some((headers, bytes))
    } else {
        None
    }
}

pub fn get_peers<'a>(bencoded_data: Option<BencodeRef<'a>>) -> Option<Vec<u8>> {
    let data = bencoded_data?;
    let dict = data.dict()?;
    let peers = dict.lookup(b"peers")?;
    let peers = BencodeRef::bytes(peers)?;
    Some(peers.to_vec())
}

pub fn peer_vec_to_list_of_ips(peer_vec: &mut Vec<u8>, torrent: &Torrent) -> Vec<Peer> {
    let mut peers_vec_sockets: Vec<Peer> = Vec::new();
    while peer_vec.len() > 0 {
        let mut ip_bytes = peer_vec.drain(..6);

        let ip: Ipv4Addr = Ipv4Addr::new(
            ip_bytes.next().unwrap(),
            ip_bytes.next().unwrap(),
            ip_bytes.next().unwrap(),
            ip_bytes.next().unwrap()
        );

        let mut port_num = (ip_bytes.next().unwrap() as u16) << 8;
        port_num = port_num | ip_bytes.next().unwrap() as u16;
        
        peers_vec_sockets.push(Peer::new(ip, port_num, decode(torrent.info_hash()).expect("Invalid Hex")));
    }

    peers_vec_sockets
}

pub fn handle_http_tracker(tracker_url: &mut Url, torrent_file: &Torrent) -> Result<TcpStream, BTError> {
    let info_hash = decode(torrent_file.info_hash()).expect("Invalid Hex");
    let info_hash = percent_encode(&info_hash, NON_ALPHANUMERIC);


    tracker_url.query_pairs_mut()
        .append_pair("peer_id", "nickrust-28jd7l931ks")
        .append_pair("port", "6881")
        .append_pair("uploaded", "0")
        .append_pair("downloaded", "0")
        .append_pair("left", &torrent_file.length.to_string())
        .append_pair("compact", "0")
        .append_key_only("info_hash");

    let tracker_host = tracker_url.host_str().ok_or(url::ParseError::EmptyHost)?;
    let tracker_port = tracker_url.port().unwrap_or(443);
    let tracker_path = tracker_url.path();
    let tracker_queries = &format!("{}={}",tracker_url.query().unwrap(), info_hash);

    let mut stream = net::TcpStream::connect(format!("{}:{}", tracker_url.domain().unwrap(), tracker_port))?;

    let req = format!(
        "GET {}?{} HTTP/1.1\r\n\
        Host: {}\r\n\
        User-Agent: nickRust\r\n\
        Connection: close\r\n\
        \r\n",
        tracker_path, tracker_queries, tracker_host
    );

    println!("{}", req);
    stream.write_all(req.as_bytes())?;

    Ok(stream)
}

pub fn handle_https_tracker(tracker_url: &mut Url, torrent_file: &Torrent) -> Result<TlsStream<TcpStream>, BTError> {
    let info_hash = decode(torrent_file.info_hash()).expect("Invalid Hex");
    let info_hash = percent_encode(&info_hash, NON_ALPHANUMERIC);


    tracker_url.query_pairs_mut()
        .append_pair("peer_id", "nickrust-28jd7l931ks")
        .append_pair("port", "6881")
        .append_pair("uploaded", "0")
        .append_pair("downloaded", "0")
        .append_pair("left", &torrent_file.length.to_string())
        .append_pair("compact", "0")
        .append_key_only("info_hash");

    let tracker_host = tracker_url.host_str().ok_or(url::ParseError::EmptyHost)?;
    let tracker_port = tracker_url.port().unwrap_or(443);
    let tracker_path = tracker_url.path();
    let tracker_queries = &format!("{}={}",tracker_url.query().unwrap(), info_hash);

    let connector = TlsConnector::new().unwrap();
    let stream = net::TcpStream::connect(format!("{}:{}", tracker_url.domain().unwrap(), tracker_port))?;
    let mut stream = connector.connect(tracker_url.domain().unwrap(), stream)?;

    let req = format!(
        "GET {}?{} HTTP/1.1\r\n\
        Host: {}\r\n\
        User-Agent: nickRust\r\n\
        Connection: close\r\n\
        \r\n",
        tracker_path, tracker_queries, tracker_host
    );

    println!("{}", req);
    stream.write_all(req.as_bytes())?;

    Ok(stream)
}