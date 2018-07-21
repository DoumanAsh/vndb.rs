//! Tokio base client

extern crate tokio;
extern crate tokio_io;
extern crate tokio_codec;
extern crate tokio_rustls;
extern crate webpki_roots;
extern crate futures;

use ::protocol;
use super::{API_HOST, API_SSL_PORT};

use std::time;
use std::convert;
use std::net;
use std::io;
use std::sync;

use self::futures::{Future, Stream, Sink};
use self::futures::sync::{mpsc};
use self::tokio_codec::{Framed, Decoder};

type VndbFramed = Framed<tokio_rustls::TlsStream<tokio::net::TcpStream, tokio_rustls::rustls::ClientSession>, protocol::Codec>;

type VndbSink = futures::stream::SplitSink<VndbFramed>;
///VNDB's Stream alias.
///
///It is a stream with [Response](../../protocol/message/enum.Response.html)
pub type VndbStream = futures::stream::SplitStream<VndbFramed>;

#[derive(Clone)]
///Send component of VNDB Client.
pub struct ClientSender(mpsc::UnboundedSender<protocol::message::Request>);

impl ClientSender {
    ///Performs send of request
    ///
    ///It can fail only when connection gets closed.
    ///Which means VNDB Client is no longer valid.
    pub fn request<M: convert::Into<protocol::message::Request>>(&self, message: M) -> Result<(), io::Error> {
        self.0.unbounded_send(message.into())
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
        Self { sender, stream }
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

    #[inline]
    ///Creates future with default settings that attempts to connect to VNDB.
    ///
    ///Once resolved it spawns tokio's job that handles sends of all messages.
    ///As soon as tokio's runtime will be stopped, the client will be invalidated.
    ///
    ///For info on default settings see [Builder](struct.Builder.html)
    pub fn new() -> io::Result<PendingConnect> {
        Builder::new().build()
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
    ///Connection stage of future.
    ///
    ///During this stage it attempts to connect to VNDB using first found address within specified
    ///deadline.
    ///In case of failure it shall try another address, if available.
    ///Otherwise it returns `io::Error`.
    ///Examine error to determine cause.
    Connecting(Option<Vec<net::SocketAddr>>, tokio::timer::Deadline<tokio::net::ConnectFuture>, u64),
    ///Establishing TLS connection stage.
    ///
    ///As connection is already established there is no deadline.
    TlsConnect(tokio_rustls::ConnectAsync<tokio::net::TcpStream>),
}

impl Future for PendingConnect {
    type Item = Client;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        loop {
            let new_state = match self {
                PendingConnect::Connecting(addrs, fut, deadline) => match fut.poll() {
                    Ok(connect) => {
                        use self::tokio_rustls::{ClientConfigExt, rustls::ClientConfig, webpki::DNSNameRef};

                        let connect = is_async!(connect);

                        let mut config = ClientConfig::new();
                        config.root_store.add_server_trust_anchors(&self::webpki_roots::TLS_SERVER_ROOTS);
                        let config = sync::Arc::new(config);
                        let domain = DNSNameRef::try_from_ascii_str(API_HOST).expect("Parse VNDB domain");
                        let tls_connect = config.connect_async(domain, connect);
                        PendingConnect::TlsConnect(tls_connect)
                    },
                    Err(error) => match addrs.as_mut().and_then(|addrs| addrs.pop()) {
                        Some(addr) => {
                            let addrs = addrs.take();
                            let connect = tokio::net::TcpStream::connect(&addr);
                            let connect = tokio::timer::Deadline::new(connect, time::Instant::now() + time::Duration::from_millis(*deadline));
                            PendingConnect::Connecting(addrs, connect, *deadline)
                        },
                        None => return match error.is_elapsed() {
                            //is_elapsed returns whether future failed to compute in time
                            true => Err(io::Error::new(io::ErrorKind::TimedOut, "Connection to VNDB timed out")),
                            false => Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "Unable to connect to VNDB IP"))
                        }
                    }
                },
                PendingConnect::TlsConnect(fut) => {
                    let socket = is_async!(fut.poll().map_err(|error| io::Error::new(io::ErrorKind::Other, error))?);
                    let (sender, receiver) = protocol::Codec::new().framed(socket).split();
                    let client = Client::internal_new(sender, receiver);
                    return Ok(futures::Async::Ready(client));
                }
            };

            *self = new_state;
        }
    }
}

///VNDB Client builder
pub struct Builder {
    connect_deadline: u64
}

impl Builder {
    ///Creates builder with default values
    ///
    ///- `connect_deadline` is `2s`
    pub fn new() -> Self {
        Self {
            connect_deadline: 2_000
        }
    }

    ///Sets deadline value milliseconds for establishment of TCP connection.
    ///
    ///Note that this time is for each attempt to establish TCP connection,
    ///in case VNDB would have multiple IP addresses.
    pub fn connect_deadline(mut self, new_deadline: u64) -> Self {
        self.connect_deadline = new_deadline;
        self
    }

    ///Starts [PendingConnect](enum.PendingConnect.html)
    pub fn build(self) -> io::Result<PendingConnect> {
        use self::net::ToSocketAddrs;

        let mut addrs = (API_HOST, API_SSL_PORT).to_socket_addrs()?;
        let (addrs, first) = match addrs.next() {
            Some(first) => {
                let addrs = addrs.collect::<Vec<_>>();
                match addrs.len() {
                    0 => (None, first),
                    _ => (Some(addrs), first)
                }
            },
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "No available IPs for VNDB"))
        };

        let connect = tokio::net::TcpStream::connect(&first);
        let connect = tokio::timer::Deadline::new(connect, time::Instant::now() + time::Duration::from_millis(self.connect_deadline));
        Ok(PendingConnect::Connecting(addrs, connect, self.connect_deadline))
    }
}
