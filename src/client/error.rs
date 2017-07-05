//! Possible errors for VNDB Client

use ::net;
use ::io;
use ::native_tls;
use ::fmt;

#[derive(Deserialize, Debug)]
///API Error
///
///VNDB API [Reference](https://vndb.org/d11#7)
pub struct VndbError {
    pub id: String,
    ///Message
    ///
    ///Note that the value of "msg" is not directly linked to the error identifier
    pub msg: String
}

impl fmt::Display for VndbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error(id='{}')={}", self.id, self.msg)
    }
}

error_chain! {
    foreign_links {
        Io(io::Error) #[doc = "IO errors"];
        Tls(native_tls::Error) #[doc = "TLS related errors."];
        TlsHandshake(native_tls::HandshakeError<net::TcpStream>) #[doc = "TLS handshake error that happen only durring initial connection."];
    }

    errors {
        Vndb(t: VndbError) {
            description("VNDB returned error")
            display("VNDB error: {}", t)
        }
    }
}

