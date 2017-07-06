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

    ///Reads message from VNDB server. Blocks.
    ///
    ///Response format: `<command> [payload]`
    ///
    ///This function on success returns string with payload.
    pub fn read_msg(&mut self) -> error::Result<Option<String>> {
        let mut stream = io::BufReader::new(&mut self.stream);
        let mut buffer = Vec::with_capacity(2);

        stream.read_until(4, &mut buffer)?;

        let _ = buffer.pop(); //remove 0x04

        //I'm pretty sure VNDB is likely to send bad string
        let msg = String::from_utf8(buffer).expect("Failed to convert response to string!");
        let mut split_msg = msg.splitn(2, ' ');

        let command = split_msg.next().expect("There should be at least 1 word in response!");
        let data = split_msg.next();

        if command == "error" {
            //There is no point in empty error.
            //Hopefully VNDB developers ain't that lazy.
            let data = data.expect("Empty error from VNDB!");

            Err(error::parse_vndb_error(data).expect("Unable to parse VNDB error! Bad VNDB API developer!"))
        }
        else {
            //Pretty sure only 'ok' response can be with empty payload
            //TODO: return pair (EnumVariant, String) ?
            Ok(data.map(|data| data.to_string()))
        }
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
