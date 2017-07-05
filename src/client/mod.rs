//! Client module

use ::serde;

use ::native_tls::{
    TlsConnector,
    TlsStream
};

use ::io;
use io::{BufRead, Write};
use ::net;

use ::message;

pub mod error;

///Client to interact with VNDB API.
pub struct Client {
    stream: TlsStream<net::TcpStream>
}

impl Client {
    ///Creates new instance of client.
    ///
    ///Upon creation connection happens.
    pub fn new() -> error::Result<Self> {
        let connector = TlsConnector::builder()?.build()?;
        let stream = net::TcpStream::connect("api.vndb.org:19535")?;
        let stream = connector.connect("api.vndb.org", stream)?;

        Ok(Client {
            stream: stream
        })
    }

    ///Sends message toward VNDB server.
    pub fn send_msg<T: serde::Serialize>(&mut self, login: &str, args: Option<T>) -> io::Result<()> {
        self.stream.write_all(message::construct(login, Some(args)).as_bytes())
    }

    ///Reads message from VNDB server. Blocks
    pub fn read_msg(&mut self) -> io::Result<String> {
        let mut stream = io::BufReader::new(&mut self.stream);
        let mut buffer = Vec::with_capacity(2);

        stream.read_until(4, &mut buffer)?;

        //TODO: parse Ok/Error
        Ok(String::from_utf8(buffer).expect("Failed to convert response to string"))
    }

    ///Sends login command
    pub fn login(&mut self, args: Option<message::args::Login>) -> error::Result<()> {
        let args = match args {
            Some(args) => args,
            None => message::args::Login::default()

        };

        self.send_msg("login", Some(args))?;
        self.read_msg()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect() {
        let client = Client::new();

        assert!(client.is_ok());

        let mut client = client.unwrap();

        client.login(None).unwrap();
    }
}
