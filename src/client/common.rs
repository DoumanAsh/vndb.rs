//!Common stuff for client modules.
extern crate tokio_io;

use tokio_io::codec::{Encoder, Decoder};

use ::net::{SocketAddr, ToSocketAddrs};

pub const API_DOMAIN: &'static str = "api.vndb.org";

lazy_static! {
    pub static ref API_URL: SocketAddr = "api.vndb.org:19535".to_socket_addrs().unwrap().next().unwrap();
}

pub type Request = <::protocol::Codec as Encoder>::Item;
pub type Response = <::protocol::Codec as Decoder>::Item;
