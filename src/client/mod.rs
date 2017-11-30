//!VNDB API Client
//!
extern crate tokio_tls;
extern crate native_tls;
use ::protocol;

use futures::{Future};
use ::futures;
use tokio_core::{reactor, net};
use tokio_io::{AsyncRead};
use tokio_io::codec::{Framed, Encoder};
use self::native_tls::{TlsConnector};
use self::tokio_tls::{TlsConnectorExt};

use ::net::{SocketAddr, ToSocketAddrs};
use ::io;

#[macro_use]
mod utils;
use self::utils::IoError;

const API_DOMAIN: &'static str = "api.vndb.org";

lazy_static! {
    static ref API_URL: SocketAddr = "api.vndb.org:19535".to_socket_addrs().unwrap().next().unwrap();
}

///Pending connection to VNDB API server.
pub struct Connect {
    inner: Box<Future<Error = io::Error, Item = tokio_tls::TlsStream<net::TcpStream>> + Send>
}

impl Future for Connect {
    type Item = Client;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let framed = try_ready!(self.inner.poll()).framed(protocol::Codec);

        Ok(futures::Async::Ready(Client {
            framed
        }))
    }
}

///VNDB API Client
pub struct Client {
    framed: Framed<tokio_tls::TlsStream<net::TcpStream>, protocol::Codec>
}

impl Client {
    ///Creates future that resolves to Client on successful connection.
    pub fn new(handle: &reactor::Handle) -> io::Result<Connect> {
        let cx = TlsConnector::builder().map_io()?.build().map_io()?;
        let socket = net::TcpStream::connect(&API_URL, handle).map_err(map_io!());

        let inner = socket.and_then(move |socket| cx.connect_async(API_DOMAIN, socket).map_err(map_io!()));

        Ok(Connect {
            inner: Box::new(inner)
        })
    }

    pub fn send(&self, message: <protocol::Codec as Encoder>::Item) {
        //TODO: consider how to wrap sending process properly.
    }
}
