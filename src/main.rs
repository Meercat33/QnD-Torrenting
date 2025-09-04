mod errors;
mod helpers;

use lava_torrent::torrent::v1::Torrent;
use std::{io::{Read, Write}};
use url::Url;
use errors::BTError;
use helpers::{get_peers, handle_http_tracker, handle_https_tracker, split_http_from_bencoded_bytes, peer_vec_to_list_of_ips};
use bip_bencode::{BDecodeOpt, BencodeRef};

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

fn main() -> Result<(), BTError> {
    let torrent = Torrent::read_from_file("/Users/nick/Documents/Coding/Rust/torrenting/sample2.torrent").unwrap();

    let tracker = torrent.announce.as_ref().unwrap();
    let mut parsed_tracker_url = Url::parse(tracker)?;

    let mut stream: Box<dyn ReadWrite> = if parsed_tracker_url.scheme() == "https" {
        Box::new(handle_https_tracker(&mut parsed_tracker_url, torrent)?)
    } else if parsed_tracker_url.scheme() == "http" {
        Box::new(handle_http_tracker(&mut parsed_tracker_url, torrent)?)
    } else {
        panic!("Invalid tracker scheme: {}", parsed_tracker_url.scheme());
    };

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    let split_response = split_http_from_bencoded_bytes(&response)
        .ok_or_else(|| 
        std::io::Error::new(
            std::io::ErrorKind::Other, "No Bencoded bytes"
        )
    )?;

    let peers = get_peers(BencodeRef::decode(split_response.1, BDecodeOpt::default()).ok()); // ERR: No Peers Connected

    match peers {
        Some(mut p) => {
            peer_vec_to_list_of_ips(&mut p);
        }
        None => { println!("No Peers Connected") }
    }

    Ok(())

}   