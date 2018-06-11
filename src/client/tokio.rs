//! Tokio base client

extern crate tokio;
extern crate tokio_io;
extern crate tokio_tls;
extern crate native_tls;
extern crate futures;

use ::protocol;
use super::{API_HOST, API_SSL_PORT};

use std::convert;
use std::net;
use std::io;

use self::futures::{Future, Stream, Sink};
use self::futures::sync::{mpsc};
use self::tokio_tls::{TlsConnectorExt};
use self::native_tls::{TlsConnector};
use self::tokio_io::AsyncRead;
use self::tokio_io::codec::Framed;

mod trust_dns {
    extern crate trust_dns_resolver;

    pub use self::trust_dns_resolver::ResolverFuture;
    pub use self::trust_dns_resolver::error::ResolveError;
    pub use self::trust_dns_resolver::config::{
        ResolverConfig,
        ResolverOpts
    };
    pub use self::trust_dns_resolver::lookup_ip::{
        LookupIpFuture
    };
}

type VndbFramed = Framed<tokio_tls::TlsStream<tokio::net::TcpStream>, protocol::Codec>;

type VndbSink = futures::stream::SplitSink<VndbFramed>;
type VndbStream = futures::stream::SplitStream<VndbFramed>;

#[derive(Clone)]
///Send component of VNDB Client.
pub struct ClientSender(mpsc::UnboundedSender<protocol::message::Request>);

impl ClientSender {
    ///Performs send of request
    ///
    ///It can fail only when connection gets closed.
    ///Which means VNDB Client is no longer valid.
    pub fn request<M: convert::Into<protocol::message::Request>>(&self, message: M) -> Result<(), io::Error> {
        self.0
            .unbounded_send(message.into())
            .map_err(Self::broken_pipe)
    }

    #[inline]
    fn broken_pipe<T>(_: T) -> io::Error {
        io::Error::new(io::ErrorKind::BrokenPipe, "No longer able to send messages")
    }
}

impl Sink for ClientSender {
    type SinkItem = protocol::message::Request;
    type SinkError = io::Error;

    #[inline]
    fn start_send(&mut self, msg: Self::SinkItem) -> futures::StartSend<Self::SinkItem, Self::SinkError> {
        self.0.start_send(msg).map_err(Self::broken_pipe)
    }

    #[inline]
    fn poll_complete(&mut self) -> futures::Poll<(), Self::SinkError> {
        self.0.poll_complete().map_err(Self::broken_pipe)
    }

    #[inline]
    fn close(&mut self) -> futures::Poll<(), io::Error> {
        Ok(futures::Async::Ready(()))
    }
}

///Tokio based VNDB API Client
pub struct Client {
    sender: ClientSender,
    stream: VndbStream
}

impl Client {
    fn internal_new(sink: VndbSink, stream: VndbStream) -> Self {
        let (sender, receiver) = mpsc::unbounded();

        let receiver = receiver.map_err(|_| io::Error::new(io::ErrorKind::Other, "Broken vndb worker chan"));
        let work = sink.send_all(receiver)
                       .map(|_| ())
                       .map_err(|error| panic!("Worker error: {}", error));

        tokio::spawn(work);

        Self::from_parts(ClientSender(sender), stream)
    }

    #[inline]
    ///Constructs client from its components
    pub fn from_parts(sender: ClientSender, stream: VndbStream) -> Self {
        Self {
            sender,
            stream
        }
    }

    #[inline]
    ///Splits client into its underlying components
    ///
    ///Note: When you split apart client, you must ensure
    ///that both ends are alive otherwise connection will be closed
    pub fn into_parts(self) -> (ClientSender, VndbStream) {
        (self.sender, self.stream)
    }

    #[inline]
    ///Retrieves sender channel
    pub fn sender(&self) -> ClientSender {
        self.sender.clone()
    }

    ///Creates future that attempts to connect to VNDB.
    ///
    ///Once resolved it spawns tokio's job that handles sends of all messages.
    ///As soon as tokio's runtime will be stopped, the client will be invalidated.
    ///
    ///In impossible case of underlying mpsc to throw error, the job shall panic.
    pub fn new() -> PendingConnect {
        let resolve = trust_dns::ResolverFuture::new(trust_dns::ResolverConfig::default(), trust_dns::ResolverOpts::default());
        PendingConnect::CreateResolver(resolve)
    }

    #[inline]
    ///Performs send of request
    ///
    ///It can fail only when connection gets closed.
    ///Which means VNDB Client is no longer valid.
    pub fn send<M: convert::Into<protocol::message::Request>>(&self, message: M) -> Result<(), io::Error> {
        self.sender.request(message)
    }
}

impl Stream for Client {
    type Item = protocol::message::Response;
    type Error = io::Error;


    fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
        self.stream.poll()
    }
}

macro_rules! is_async {
    ($result:expr) => ({
        match $result {
            futures::Async::Ready(result) => result,
            futures::Async::NotReady => return Ok(futures::Async::NotReady)
        }
    })
}

///Pending establishment of connect toward VNDB server.
#[must_use = "Must be polled!"]
pub enum PendingConnect {
    #[doc(hidden)]
    CreateResolver(Box<Future<Item=trust_dns::ResolverFuture, Error=trust_dns::ResolveError> + Send>),
    #[doc(hidden)]
    LookupIpFuture(trust_dns::LookupIpFuture),
    #[doc(hidden)]
    LookupIp(Option<Vec<net::IpAddr>>, tokio::net::ConnectFuture),
    #[doc(hidden)]
    TlsConnect(tokio_tls::ConnectAsync<tokio::net::TcpStream>),
}

impl Future for PendingConnect {
    type Item = Client;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        loop {
            let new_state = match self {
                PendingConnect::CreateResolver(fut) => {
                    let resolver = is_async!(fut.poll()?);
                    PendingConnect::LookupIpFuture(resolver.lookup_ip(API_HOST))
                },
                PendingConnect::LookupIpFuture(fut) => {
                    //TODO: it seems to be quite slow?
                    let ips = is_async!(fut.poll()?);
                    let mut ips = ips.iter();
                    let (ips, first_ip) = match ips.next() {
                        Some(ip) => (Some(ips.collect()), net::SocketAddr::new(ip, API_SSL_PORT)),
                        None => return Err(io::Error::new(io::ErrorKind::NotFound, "No available IPs for VNDB"))
                    };
                    PendingConnect::LookupIp(ips, tokio::net::TcpStream::connect(&first_ip))
                },
                PendingConnect::LookupIp(ips, fut) => match fut.poll() {
                    Ok(connect) => {
                        let connect = is_async!(connect);
                        let tls_context = TlsConnector::builder().map_err(|error| io::Error::new(io::ErrorKind::Other, error))?
                                                                 .build()
                                                                 .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
                        let tls_connect = tls_context.connect_async(API_HOST, connect);
                        PendingConnect::TlsConnect(tls_connect)
                    },
                    Err(_) => match ips.as_mut().unwrap().pop() {
                        Some(ip) => {
                            let addr = net::SocketAddr::new(ip, API_SSL_PORT);
                            let ips = ips.take();
                            PendingConnect::LookupIp(ips, tokio::net::TcpStream::connect(&addr))
                        },
                        None => return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "Unable to connect to VNDB IP"))
                    }
                },
                PendingConnect::TlsConnect(fut) => {
                    let socket = is_async!(fut.poll().map_err(|error| io::Error::new(io::ErrorKind::Other, error))?);
                    let (sender, receiver) = socket.framed(protocol::Codec).split();
                    let client = Client::internal_new(sender, receiver);
                    return Ok(futures::Async::Ready(client));
                }
            };

            *self = new_state;
        }
    }
}
