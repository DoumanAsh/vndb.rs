//!Tokio-based Single threaded Client API.
extern crate tokio_tls;
extern crate native_tls;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use ::protocol;

use self::futures::{Future, Sink, Stream};
use self::tokio_core::{reactor, net};
use self::tokio_io::{AsyncRead};
use self::tokio_io::codec::{Framed};
use self::native_tls::{TlsConnector};
use self::tokio_tls::{TlsConnectorExt};

use ::io;
use ::convert;
use ::std::rc::Rc;
use ::std::cell::{
    Cell,
};

use super::utils;
use self::utils::IoError;
use super::common::{
    API_DOMAIN,
    API_URL,
    Request,
    Response
};

type VndbFramed = Framed<tokio_tls::TlsStream<net::TcpStream>, protocol::Codec>;
type Writer = futures::stream::SplitSink<VndbFramed>;
type Reader = futures::stream::SplitStream<VndbFramed>;

#[must_use = "Must be polled!"]
///Represents ongoing request.
pub struct PendingRequest {
    sender: Sender,
    message: Option<Request>
}

impl Future for PendingRequest {
    type Item = Sender;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let writer = unsafe { &mut *(self.sender.inner.as_ptr()) };

        //We take out value of message leaving None
        //If sink is not able to send, value is returned back.
        match writer.start_send(self.message.take().unwrap())? {
            futures::AsyncSink::Ready => {
                writer.poll_complete()?;
                Ok(futures::Async::Ready(self.sender.clone()))
            },
            futures::AsyncSink::NotReady(message) => {
                self.message = Some(message);
                Ok(futures::Async::NotReady)
            }
        }
    }
}

#[derive(Clone)]
///Represents writer of [Client](struct.Client.html).
pub struct Sender {
    inner: Rc<Cell<Writer>>
}

impl Sender {
    fn new(writer: Writer) -> Self {
        Self {
            inner: Rc::new(Cell::new(writer))
        }
    }

    ///Starts send of VNDB request.
    pub fn send<M: convert::Into<Request>>(&self, message: M) -> PendingRequest {
        PendingRequest {
            sender: self.clone(),
            message: Some(message.into())
        }
    }
}

#[must_use = "Must be polled!"]
///Future that resolves into VNDB Response on success.
///
///If response is resolved to `None` it means that connection is closed and
///[Client](struct.Client.html) is no longer capable of communicating with VNDB.
pub struct FutureResponse {
    reader: Receiver
}

impl Future for FutureResponse {
    type Item = (Option<Response>, Receiver);
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let reader = unsafe { &mut *(self.reader.inner.as_ptr()) };
        let result = reader.poll()?;

        match result {
            futures::Async::Ready(rsp) => Ok(futures::Async::Ready((rsp, self.reader.clone()))),
            futures::Async::NotReady => Ok(futures::Async::NotReady)
        }
    }
}

#[derive(Clone)]
///Represents reader of [Client](struct.Client.html).
pub struct Receiver {
    inner: Rc<Cell<Reader>>
}

impl Receiver {
    fn new(reader: Reader) -> Self {
        Self {
            inner: Rc::new(Cell::new(reader))
        }
    }

    ///Creates future that attempts to retrieve response.
    pub fn receive(&self) -> FutureResponse {
        FutureResponse {
            reader: self.clone()
        }
    }
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
        let (writer, reader) = framed.split();

        Ok(futures::Async::Ready(Client {
            writer: Sender::new(writer),
            reader: Receiver::new(reader)
        }))
    }
}

///VNDB API Client
pub struct Client {
    writer: Sender,
    reader: Receiver
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

    #[inline]
    ///Starts sending VNDB Request toward server.
    pub fn send<M: convert::Into<Request>>(&self, message: M) -> PendingRequest {
        self.writer.send(message)
    }

    #[inline]
    ///Creates future that attempts to retrieve response.
    pub fn receive(&self) -> FutureResponse {
        self.reader.receive()
    }
}
