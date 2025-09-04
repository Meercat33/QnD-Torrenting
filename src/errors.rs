use std::net::TcpStream;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BTError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),

    #[error("TCP Handshake Error: {0}")]
    TCP(#[from] native_tls::HandshakeError<TcpStream>),

    #[error("From Hex Error: {0}")]
    HEX(#[from] hex::FromHexError)
}