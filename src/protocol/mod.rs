//!VNDB Protocol implementation.
//!
//![API Reference](https://vndb.org/d11).

use ::bytes;

use futures::{Future};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};

use ::io;

pub mod message;

///VNDB's Codec implementation.
pub struct Codec;

impl Codec {
    #[inline]
    fn to_str<'a>(msg: &'a [u8]) -> io::Result<&'a str> {
        ::str::from_utf8(&msg).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }
}

impl Decoder for Codec {
    type Item = message::Response;
    type Error = io::Error;

    fn decode(&mut self, buff: &mut bytes::BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buff.iter().position(|&b| b == 4) {
            // remove the serialized frame from the buffer alongside 0x04.
            let mut line = buff.split_to(i + 1);
            // Remove 0x04
            let line = {
                let len = line.len() - 1;
                line.split_to(len)
            };
            // Convert to utf-8 just to  be sure it is encoded properly.
            let line = Self::to_str(&line)?;

            message::Response::from_str(line).map(|result| Some(result))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for Codec {
    type Item = message::Request;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buff: &mut bytes::BytesMut) -> io::Result<()> {
        buff.extend_from_slice(format!("{}", msg).as_bytes());

        Ok(())
    }
}
