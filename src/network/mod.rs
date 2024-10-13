use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod client;
pub mod protocol;
mod server;
mod shared;

#[allow(unused_imports)]
pub use {client::*, server::*};

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
