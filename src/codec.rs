//! VNDB codec

use crate::protocol::message;

use std::io;
use core::fmt::{self, Write};

struct BytesWriter<'a> {
    buff: &'a mut bytes::BytesMut,
}

impl<'a> fmt::Write for BytesWriter<'a> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.buff.extend_from_slice(text.as_bytes());
        Ok(())
    }
}

///VNDB codec, implements tokio encoder/decoder traits
#[derive(Default)]
pub struct Codec {
    next_idx: usize,
}

impl Codec {
    #[inline]
    ///Creates default codec instance.
    pub const fn new() -> Self {
        Self {
            next_idx: 0
        }
    }
    #[inline]
    fn to_str<'a>(msg: &'a [u8]) -> io::Result<&'a str> {
        core::str::from_utf8(&msg).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Message is not encoded as UTF-8"))
    }

}

impl tokio_codec::Encoder for Codec {
    type Item = message::Request<'static>;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buff: &mut bytes::BytesMut) -> io::Result<()> {
        let mut writer = BytesWriter {
            buff
        };

        let _ = write!(writer, "{}", msg);

        Ok(())
    }
}

impl tokio_codec::Decoder for Codec {
    type Item = message::Response;
    type Error = io::Error;

    fn decode(&mut self, buff: &mut bytes::BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buff[self.next_idx..].iter().position(|&b| b == 4) {
            // remove the serialized frame from the buffer alongside 0x04.
            let mut line = buff.split_to(self.next_idx + i + 1);

            self.next_idx = 0;

            // Remove 0x04
            let line = {
                let len = line.len() - 1;
                line.split_to(len)
            };
            // Convert to utf-8 just to  be sure it is encoded properly.
            let line = Self::to_str(&line)?;

            message::Response::from_str(line).map(|result| Some(result))
                                             .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
        } else {
            self.next_idx = buff.len();
            Ok(None)
        }
    }
}
