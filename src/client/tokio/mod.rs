//!Tokio-based Client API.
//!
//!# Flavours:
//!There are two flavours:
//!
//!* [RcClient](type.RcClient.html) - Single threaded with zero overhead.
//!* [ArcClient](type.ArcClient.html) - Multi-threaded that wraps underlying Sink/Stream into mutex.
//!
//!# Usage:
//!```rust
//!#[macro_use]
//!extern crate vndb;
//!extern crate tokio_core;
//!extern crate futures;
//!
//!use futures::future::Future;
//!
//!//These traits are required when using Sender/Receiver directly from result of futures.
//!use vndb::client::tokio::{Sender, Receiver};
//!use vndb::protocol::message;
//!
//!fn main() {
//!    let mut tokio_core = tokio_core::reactor::Core::new().expect("Should create tokio core");
//!    let client = vndb::client::tokio::RcClient::new(&tokio_core.handle()).expect("Should initialize client");
//!    let client = tokio_core.run(client).expect("Pending connect should be successful");
//!    let get = message::request::Get {
//!            kind: message::request::get::Type::vn(),
//!            flags: message::request::get::Flags::new().basic().details(),
//!            filters: message::request::get::Filters::new().filter(filter!(title ~ "Kizuna")).or(filter!(title = "Kizuna")),
//!            options: None
//!    };
//!
//!    let send = client.send(message::request::Login::new(None, None)).map_err(|_| "Successful Login send")
//!                     .and_then(|sender| sender.send(message::Request::DBstats).map_err(|_| "Successful DBstats send"))
//!                     .and_then(move |sender| sender.send(get).map_err(|_| "Successful Get vn send"));
//!
//!    tokio_core.run(send).expect("Should send multiple requests!");
//!
//!    let receive = client.receive()
//!                        .and_then(|(response, receiver)| match response {
//!                            Some(message::Response::Ok) => {
//!                                println!("Logged into VNDB!");
//!                                receiver.receive()
//!                            },
//!                            _ => panic!("oi oi... I cannot login :(")
//!                        }).and_then(|(response, receiver)| match response {
//!                             Some(message::Response::DBstats(response)) => {
//!                                 println!("DBstats={:?}", response);
//!                                 receiver.receive()
//!                             },
//!                             _ => panic!("oi oi... I cannot get DBstats :(")
//!                        }).map(|(response, _)| match response {
//!                             Some(message::Response::Results(response)) => {
//!                                 response.vn().expect("Parse into VN Results")
//!                             },
//!                             _ => panic!("oi oi... I cannot get VN :(")
//!                        });
//!
//!
//!    let vn = tokio_core.run(receive).expect("Should receive multiple requests!");
//!    println!("vns={:?}", vn);
//!}
//!```

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
use ::cell::Cell;
use ::sync::{Arc, Mutex};
use ::marker::{Sized, PhantomData};

use super::utils;
use self::utils::IoError;
use super::common::{
    API_DOMAIN,
    api_url,
    Request,
    Response
};

type VndbFramed = Framed<tokio_tls::TlsStream<net::TcpStream>, protocol::Codec>;

///Async writer type.
type Writer = futures::stream::SplitSink<VndbFramed>;
///Async reader type.
type Reader = futures::stream::SplitStream<VndbFramed>;

//Basic traits
///Trait for writer of [Client](struct.Client.html).
pub trait Sender: Sized + Clone {
    ///Creates new instance.
    fn new(writer: Writer) -> Self;
    ///Starts send of VNDB request.
    fn send<M: convert::Into<Request>>(&self, message: M) -> PendingRequest<Self> {
        PendingRequest {
            sender: self.clone(),
            message: Some(message.into())
        }
    }
}

///Trait for reader of [Client](struct.Client.html).
pub trait Receiver: Sized + Clone {
    ///Creates new instance.
    fn new(reader: Reader) -> Self;
    ///Creates future that attempts to retrieve response.
    fn receive(&self) -> FutureResponse<Self> {
        FutureResponse {
            reader: self.clone()
        }
    }
}

//Futures
#[must_use = "Must be polled!"]
///Pending connection to VNDB API server.
pub struct PendingConnect<S: Sender, R: Receiver> {
    inner: Box<Future<Item = tokio_tls::TlsStream<net::TcpStream>, Error = io::Error> + Send>,
    _sender: PhantomData<S>,
    _receiver: PhantomData<R>
}

impl<S: Sender, R: Receiver> Future for PendingConnect<S, R> {
    type Item = Client<S, R>;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let framed = try_ready!(self.inner.poll()).framed(protocol::Codec);
        let (writer, reader) = framed.split();

        Ok(futures::Async::Ready(Client {
            writer: S::new(writer),
            reader: R::new(reader)
        }))
    }
}

#[must_use = "Must be polled!"]
///Future that resolves into VNDB Response on success.
///
///If response is resolved to `None` it means that connection is closed and
///[Client](struct.Client.html) is no longer capable of communicating with VNDB.
pub struct FutureResponse<R: Receiver> {
    reader: R
}

impl<R: Receiver> FutureResponse<R> where Self: Future<Item=(Option<Response>, R)> {
    #[inline]
    fn int_poll(receiver: &R, reader: &mut Reader) -> futures::Poll<<Self as Future>::Item, io::Error> {
        let result = reader.poll()?;

        match result {
            futures::Async::Ready(rsp) => Ok(futures::Async::Ready((rsp, receiver.clone()))),
            futures::Async::NotReady => Ok(futures::Async::NotReady)
        }
    }
}

impl Future for FutureResponse<RcReceiver> {
    type Item = (Option<Response>, RcReceiver);
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let reader = unsafe { &mut *(self.reader.inner.as_ptr()) };

        Self::int_poll(&self.reader, reader)
    }
}

impl Future for FutureResponse<ArcReceiver> {
    type Item = (Option<Response>, ArcReceiver);
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let mut lock = self.reader.inner.try_lock();
        let reader = match lock {
            Ok(ref mut writer) => writer,
            Err(_) => return Ok(futures::Async::NotReady)
        };

        Self::int_poll(&self.reader, reader)
    }
}

#[must_use = "Must be polled!"]
///Represents ongoing request.
pub struct PendingRequest<S: Sender> {
    sender: S,
    message: Option<Request>
}

impl<S: Sender> PendingRequest<S> where Self: Future<Item=S> {
    #[inline]
    fn int_poll(sender: &S, message: &mut Option<Request>, writer: &mut Writer) -> futures::Poll<<Self as Future>::Item, io::Error> {
        //We take out value of message leaving None
        //If sink is not able to send, value is returned back.
        match writer.start_send(message.take().unwrap())? {
            futures::AsyncSink::Ready => {
                writer.poll_complete()?;
                Ok(futures::Async::Ready(sender.clone()))
            },
            futures::AsyncSink::NotReady(returned) => {
                *message = Some(returned);
                Ok(futures::Async::NotReady)
            }
        }
    }
}

impl Future for PendingRequest<RcSender> {
    type Item = RcSender;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let writer = unsafe { &mut *(self.sender.inner.as_ptr()) };

        Self::int_poll(&self.sender, &mut self.message, writer)
    }
}

impl Future for PendingRequest<ArcSender> {
    type Item = ArcSender;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let mut lock = self.sender.inner.try_lock();
        let writer = match lock {
            Ok(ref mut writer) => writer,
            Err(_) => return Ok(futures::Async::NotReady)
        };

        Self::int_poll(&self.sender, &mut self.message, writer)
    }
}


//Senders
#[derive(Clone)]
///[Client](struct.Client.html)'s writer.
///
///Uses simple Rc to store underlying sink.
///Due to that it is not safe to use it in multi-threaded environment.
///But it provides zero overhead, aside from heap allocation, otherwise.
pub struct RcSender {
    inner: Rc<Cell<Writer>>
}

impl Sender for RcSender {
    fn new(writer: Writer) -> Self {
        Self {
            inner: Rc::new(Cell::new(writer))
        }
    }
}

#[derive(Clone)]
///[Client](struct.Client.html)'s thread-safe writer.
///
///Uses Arc to store underlying sink.
///Thanks to that can be sent to multiple threads by simply cloning itself.
///
///Mutex is used to control access, but inability to acquire lock will lead to next iteration
///of polling.
pub struct ArcSender {
    inner: Arc<Mutex<Writer>>,
}

impl Sender for ArcSender {
    fn new(writer: Writer) -> Self {
        Self {
            inner: Arc::new(Mutex::new(writer))
        }
    }
}


//Receivers
#[derive(Clone)]
///[Client](struct.Client.html)'s reader.
///
///Uses simple Rc to store underlying stream.
///Due to that it is not safe to use it in multi-threaded environment.
///But it provides zero overhead, aside from heap allocation, otherwise.
pub struct RcReceiver {
    inner: Rc<Cell<Reader>>
}

impl Receiver for RcReceiver {
    fn new(reader: Reader) -> Self {
        Self {
            inner: Rc::new(Cell::new(reader))
        }
    }
}

#[derive(Clone)]
///[Client](struct.Client.html)'s reader.
///
///Uses Arc to store underlying sink.
///Thanks to that can be sent to multiple threads by simply cloning itself.
///
///Mutex is used to control access, but inability to acquire lock will lead to next iteration
///of polling.
pub struct ArcReceiver {
    inner: Arc<Mutex<Reader>>,
}

impl Receiver for ArcReceiver {
    fn new(reader: Reader) -> Self {
        Self {
            inner: Arc::new(Mutex::new(reader))
        }
    }

    ///Creates future that attempts to retrieve response.
    fn receive(&self) -> FutureResponse<Self> {
        FutureResponse {
            reader: self.clone()
        }
    }
}

///VNDB API Client
pub struct Client<S: Sender, R: Receiver> {
    writer: S,
    reader: R
}

impl<S: Sender, R: Receiver> Client<S, R> {
    ///Creates future that resolves to Client on successful connection.
    pub fn new(handle: &reactor::Handle) -> io::Result<PendingConnect<S, R>> {
        let cx = TlsConnector::builder().map_io()?.build().map_io()?;
        let socket = net::TcpStream::connect(&api_url()?, handle).map_err(map_io!());

        let inner = socket.and_then(move |socket| cx.connect_async(API_DOMAIN, socket).map_err(map_io!()));

        Ok(PendingConnect {
            inner: Box::new(inner),
            _sender: PhantomData,
            _receiver: PhantomData
        })
    }

    #[inline]
    ///Starts sending VNDB Request toward server.
    pub fn send<M: convert::Into<Request>>(&self, message: M) -> PendingRequest<S> {
        self.writer.send(message)
    }

    #[inline]
    ///Creates future that attempts to retrieve response.
    pub fn receive(&self) -> FutureResponse<R> {
        self.reader.receive()
    }
}

///Alias to single threaded Client.
pub type RcClient = Client<RcSender, RcReceiver>;
///Alias to multi threaded Client.
pub type ArcClient = Client<ArcSender, ArcReceiver>;
