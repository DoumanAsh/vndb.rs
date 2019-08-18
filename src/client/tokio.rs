//! Tokio based client

use crate::codec::Codec;
use crate::protocol::message;

use futures_core::Stream;
use futures_sink::Sink;

use std::io;
use std::net::{ToSocketAddrs};
use core::task;
use core::future::Future;
use core::pin::{Pin};

#[macro_export]
///Await future in async context.
///
///Because `.await` is retarded.
macro_rules! matsu {
    ($exp:expr) => {
        ($exp).await
    }
}

type Request = <Codec as tokio_codec::Encoder>::Item;

/// Tokio based client
pub struct Client<T> {
    inner: tokio_codec::Framed<T, Codec>,
}

impl<T: tokio_io::AsyncWrite + tokio_io::AsyncRead + Unpin> Client<T> {
    ///Creates Client from existing IO objects.
    pub fn from_io(io: T) -> Self {
        Self {
            inner: tokio_codec::Framed::new(io, Codec::new()),
        }
    }

    ///Reads next response
    ///
    ///`None` indicates that `Stream` has ended and requires you to reqreate client.
    pub async fn receive(&mut self) -> Option<io::Result<message::Response>> {
        let next = Next {
            stream: &mut self.inner,
        };

        matsu!(next)
    }

    ///Sends request to the server.
    ///
    ///To guarantee the send, call `flush`.
    pub async fn send(&mut self, msg: Request) -> io::Result<()> {
        let send = Send {
            sink: &mut self.inner,
            pending: Some(msg.into()),
        };

        matsu!(send)
    }

    ///Flushes all buffered requests.
    pub async fn flush(&mut self) -> io::Result<()> {
        let flush = Flush {
            sink: &mut self.inner,
        };

        matsu!(flush)
    }
}


async fn connect_tcp(host: &str, port: u16) -> io::Result<tokio_net::tcp::TcpStream> {
    let addrs = (host, port).to_socket_addrs()?;

    for addr in addrs {
        match matsu!(tokio_net::tcp::TcpStream::connect(&addr)) {
            Ok(io) => return Ok(io),
            Err(_) => continue,
        }
    }

    return Err(io::Error::new(io::ErrorKind::NotFound, "Unable to connect to api.vndb.org"));
}

impl Client<tokio_net::tcp::TcpStream> {
    #[inline]
    ///Connects to vndb server using plain TCP
    ///
    ///If provided, uses `addr` instead of default `api.vndb.org:19534`
    pub async fn connect_tcp() -> io::Result<Self> {
        Ok(Self::from_io(matsu!(connect_tcp(super::API_HOST, super::API_PORT))?))
    }
}

#[cfg(feature = "rustls")]
impl Client<tokio_rustls::client::TlsStream<tokio_net::tcp::TcpStream>> {
    #[inline]
    ///Connects to vndb server using plain TCP
    ///
    ///If provided, uses `addr` instead of default `api.vndb.org:19535`
    pub async fn connect_tls() -> io::Result<Self> {
        use tokio_rustls::{TlsConnector, rustls::ClientConfig, webpki::DNSNameRef};

        let tcp = matsu!(connect_tcp(super::API_HOST, super::API_SSL_PORT))?;

        let mut config = ClientConfig::new();
        config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let config = TlsConnector::from(std::sync::Arc::new(config));
        let domain = DNSNameRef::try_from_ascii_str(super::API_HOST).expect("Parse VNDB domain");
        let io = matsu!(config.connect(domain, tcp))?;

        Ok(Self::from_io(io))
    }
}

struct Next<'a, T> {
    stream: &'a mut T,
}

impl<T: Stream + Unpin> Future for Next<'_, T> {
    type Output = Option<T::Item>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let stream = Pin::new(&mut this.stream);
        Stream::poll_next(stream, ctx)
    }
}

struct Send<'a, T> {
    sink: &'a mut T,
    pending: Option<Request>,
}

impl<'a, T: Sink<Request> + Unpin> Future for Send<'a, T> {
    type Output = Result<(), T::Error>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        match self.pending.take() {
            Some(msg) => {
                let this = &mut *self;
                match Sink::poll_ready(Pin::new(&mut this.sink), ctx)? {
                    task::Poll::Ready(_) => task::Poll::Ready(Sink::start_send(Pin::new(&mut this.sink), msg)),
                    task::Poll::Pending => {
                        self.pending = Some(msg);
                        task::Poll::Pending
                    }
                }
            },
            None => task::Poll::Ready(Ok(())),
        }
    }
}

struct Flush<'a, T> {
    sink: &'a mut T,
}

impl<'a, T: Sink<Request> + Unpin> Future for Flush<'a, T> {
    type Output = Result<(), T::Error>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        Sink::poll_flush(Pin::new(&mut this.sink), ctx)
    }
}
