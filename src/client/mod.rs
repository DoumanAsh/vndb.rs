//!VNDB API Client
//!
extern crate tokio_tls;
extern crate native_tls;
use ::protocol;

use futures::{Future, Sink, Stream};
use ::futures;
use tokio_core::{reactor, net};
use tokio_io::{AsyncRead};
use tokio_io::codec::{Framed, Encoder, Decoder};
use self::native_tls::{TlsConnector};
use self::tokio_tls::{TlsConnectorExt};

use ::net::{SocketAddr, ToSocketAddrs};
use ::io;
use ::convert;

#[macro_use]
mod utils;
use self::utils::IoError;

const API_DOMAIN: &'static str = "api.vndb.org";

lazy_static! {
    static ref API_URL: SocketAddr = "api.vndb.org:19535".to_socket_addrs().unwrap().next().unwrap();
}

#[must_use = "Must be polled!"]
///Pending connection to VNDB API server.
pub struct PendingConnect {
    inner: Box<Future<Item = tokio_tls::TlsStream<net::TcpStream>, Error = io::Error> + Send>
}

impl Future for PendingConnect {
    type Item = Client;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let framed = try_ready!(self.inner.poll()).framed(protocol::Codec);

        Ok(futures::Async::Ready(Client {
            framed
        }))
    }
}

type ClientFramed = Framed<tokio_tls::TlsStream<net::TcpStream>, protocol::Codec>;

///VNDB API Client
pub struct Client {
    framed: ClientFramed
}

impl Client {
    ///Creates future that resolves to Client on successful connection.
    pub fn new(handle: &reactor::Handle) -> io::Result<PendingConnect> {
        let cx = TlsConnector::builder().map_io()?.build().map_io()?;
        let socket = net::TcpStream::connect(&API_URL, handle).map_err(map_io!());

        let inner = socket.and_then(move |socket| cx.connect_async(API_DOMAIN, socket).map_err(map_io!()));

        Ok(PendingConnect {
            inner: Box::new(inner)
        })
    }

    ///Performs sends of message toward server.
    ///
    ///If error is returned then, message should be re-sent.
    pub fn send<M: convert::Into<<protocol::Codec as Encoder>::Item>>(&mut self, message: M) -> io::Result<&mut Self> {
        match self.framed.start_send(message.into())? {
            futures::AsyncSink::Ready => self.framed.poll_complete()?,
            futures::AsyncSink::NotReady(_) => return Err(utils::error("Unable to send request! Sink is not ready."))
        };
        Ok(self)
    }

    ///Creates future that attempts to retrieve response.
    pub fn receive(&mut self) -> FutureResponse {
        FutureResponse {
            framed: &mut self.framed
        }
    }

    #[inline]
    ///Sends message and returns future that resolves into response.
    ///
    ///Note: If any prior requests were made using normal `send` you might get unexpected response.
    pub fn send_w_response<M: convert::Into<<protocol::Codec as Encoder>::Item>>(&mut self, message: M) -> io::Result<FutureResponse> {
        self.send(message)?;
        Ok(self.receive())
    }
}

#[must_use = "Must be polled!"]
///Future that resolves into VNDB Response on success.
pub struct FutureResponse<'a> {
    framed: &'a mut ClientFramed
}

impl<'a> Future for FutureResponse<'a> {
    type Item = Option<<protocol::Codec as Decoder>::Item>;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.framed.poll()
    }
}
