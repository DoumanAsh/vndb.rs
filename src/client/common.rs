//!Common stuff for client modules.
extern crate tokio_io;

use tokio_io::codec::{Encoder, Decoder};

use ::io;
use ::net::{SocketAddr, ToSocketAddrs};

pub const API_DOMAIN: &'static str = "api.vndb.org";
pub const API_URL: &'static str = "api.vndb.org:19535";

pub type Request = <::protocol::Codec as Encoder>::Item;
pub type Response = <::protocol::Codec as Decoder>::Item;

#[inline]
pub fn api_url() -> io::Result<SocketAddr> {
    match API_URL.to_socket_addrs()?.next() {
        Some(result) => Ok(result),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Unable to DNS query api.vndb.org"))
    }
}
