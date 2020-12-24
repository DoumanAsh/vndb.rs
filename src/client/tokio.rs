//!Client implementation of VNDB client, based on [tokio](https://github.com/tokio-rs/tokio)

use core::future::Future;

use tokio::net;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

use super::{API_HOST, API_PORT};
use crate::utils::AsPin;

///Tokio based VNDB Client
pub struct Client<IO> where IO: AsyncRead {
    io: BufReader<IO>,
    read_buf: Vec<u8>,
}

impl Client<net::TcpStream> {
    fn socket_connect() -> impl Future<Output=io::Result<net::TcpStream>> {
        net::TcpStream::connect((API_HOST, API_PORT))
    }

    #[inline(always)]
    ///Connects over plain TCP
    pub async fn connect() -> io::Result<Self> {
        Ok(Self::new(Self::socket_connect().await?))
    }

    ///Re-connects over plain TCP, aborting previous connection if any
    pub async fn reconnect(&mut self) -> io::Result<()> {
        let _ = self.io.get_mut().shutdown();

        self.io = BufReader::new(Self::socket_connect().await?);
        self.read_buf.clear();
        Ok(())
    }
}


#[cfg(feature = "rustls-on")]
impl Client<tokio_rustls::client::TlsStream<net::TcpStream>> {
    async fn socket_connect_tls() -> io::Result<tokio_rustls::client::TlsStream<net::TcpStream>> {
        let socket = net::TcpStream::connect((API_HOST, super::API_SSL_PORT)).await?;

        let dns_name = webpki::DNSNameRef::try_from_ascii_str(API_HOST).unwrap();
        let config = tokio_rustls::TlsConnector::from(super::get_rustls_config().clone());

        Ok(config.connect(dns_name, socket).await?)
    }

    #[inline(always)]
    ///Connects with TLS
    pub async fn connect_tls() -> io::Result<Self> {
        Ok(Self::new(Self::socket_connect_tls().await?))
    }

    ///Re-connects over TLS, aborting previous connection if any
    pub async fn reconnect_tls(&mut self) -> io::Result<()> {
        let _ = self.io.get_mut().get_mut().0.shutdown();

        self.io = BufReader::new(Self::socket_connect_tls().await?);
        self.read_buf.clear();
        Ok(())
    }
}

impl<IO: AsyncRead + AsyncWrite> Client<IO> {
    #[inline]
    ///Sends request to the server
    pub async fn send(&mut self, req: &crate::protocol::Request<'_>) -> io::Result<()> {
        let io = self.io.as_pin();
        BufReader::get_pin_mut(io).write_all(req.to_string().as_bytes()).await
    }

    #[inline]
    ///Flushes sent requests
    pub async fn flush(&mut self) -> io::Result<()> {
        let io = self.io.as_pin();
        BufReader::get_pin_mut(io).flush().await
    }
}

impl<IO: AsyncRead> Client<IO> {
    ///Creates new instance from existing IO object
    pub fn new(io: IO) -> Self {
        Self {
            io: BufReader::new(io),
            read_buf: Vec::new(),
        }
    }

    ///Reads single incoming response.
    ///
    ///If `None` is returned, then it means connection is closed.
    pub async fn receive(&mut self) -> io::Result<Option<crate::protocol::Response>> {
        let mut io = self.io.as_pin();

        let size = io.read_until(0x04, &mut self.read_buf).await?;

        if size == 0 {
            return Ok(None);
        }

        let result = super::parse_response(&self.read_buf);
        self.read_buf.clear();
        result
    }
}
