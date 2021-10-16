//!Simple client implementation of VNDB client, based on std's sockets

use std::{io, net};
use std::io::{Read, Write, BufRead, BufReader};

use super::{API_HOST, API_PORT};

///Simple synchronous Client implementation
pub struct Client<IO> where IO: Read {
    io: BufReader<IO>,
    read_buf: Vec<u8>,
}

impl Client<net::TcpStream> {
    fn socket_connect() -> io::Result<net::TcpStream> {
        net::TcpStream::connect((API_HOST, API_PORT))
    }

    #[inline(always)]
    ///Connects over plain TCP
    pub fn connect() -> io::Result<Self> {
        Ok(Self::new(Self::socket_connect()?))
    }

    ///Re-connects over plain TCP, aborting previous connection if any
    pub fn reconnect(&mut self) -> io::Result<()> {
        let _ = self.io.get_mut().shutdown(std::net::Shutdown::Both);

        self.io = BufReader::new(Self::socket_connect()?);
        self.read_buf.clear();
        Ok(())
    }
}

#[cfg(feature = "rustls-on")]
impl Client<rustls::StreamOwned<rustls::ClientConnection, net::TcpStream>> {
    fn socket_connect_tls() -> io::Result<rustls::StreamOwned<rustls::ClientConnection, net::TcpStream>> {
        let socket = std::net::TcpStream::connect((API_HOST, super::API_SSL_PORT))?;

        let (dns_name, config) = super::get_rustls_config();
        let sess = match rustls::ClientConnection::new(config, dns_name) {
            Ok(sess) => sess,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
        };

        Ok(rustls::StreamOwned::new(sess, socket))
    }

    #[inline(always)]
    ///Connects with TLS
    pub fn connect_tls() -> io::Result<Self> {
        Ok(Self::new(Self::socket_connect_tls()?))
    }

    ///Re-connects over TLS, aborting previous connection if any
    pub fn reconnect_tls(&mut self) -> io::Result<()> {
        let _ = self.io.get_mut().sock.shutdown(std::net::Shutdown::Both);

        self.io = BufReader::new(Self::socket_connect_tls()?);
        self.read_buf.clear();
        Ok(())
    }
}

impl<IO: Read + Write> Client<IO> {
    #[inline]
    ///Sends request to the server
    pub fn send(&mut self, req: &crate::protocol::Request) -> io::Result<()> {
        self.io.get_mut().write_fmt(format_args!("{}", req))
    }

    #[inline]
    ///Flushes sent requests
    pub fn flush(&mut self) -> io::Result<()> {
        self.io.get_mut().flush()
    }
}

impl<IO: Read> Client<IO> {
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
    pub fn receive(&mut self) -> io::Result<Option<crate::protocol::Response>> {
        let size = self.io.read_until(0x04, &mut self.read_buf)?;

        if size == 0 {
            return Ok(None);
        }

        let result = super::parse_response(&self.read_buf);
        self.read_buf.clear();
        result
    }
}
