//!VNDB message.

use ::serde_json;

use ::fmt;
use ::io;

pub mod request;
pub mod response;

///VNDB Request
///
///On error returns [Response::Error](response/Struct.VndbError.html).
pub enum Request {
    ///Login request.
    ///
    ///On success returns `Response::Ok`
    Login(request::Login),
    ///Get request.
    ///
    ///On success returns [Response::Results](response/Struct.Results.html)
    Get(request::Get),
    ///VNDB statistic request.
    ///
    ///On success returns [Response::DBstats](response/Struct.DBstats.html)
    DBstats
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Request::Login(ref login) => write!(f, "{}\x04", login),
            &Request::Get(ref get) => write!(f, "{}\x04", get),
            &Request::DBstats => write!(f, "dbstats\x04")
        }
    }
}

///VNDB Response
pub enum Response {
    ///Request is ok
    Ok,
    ///Response to Get command with data.
    Results(response::Results),
    ///DB statistic response.
    DBstats(response::DBstats),
    ///VNDB Error in case of invalid request.
    Error(response::VndbError)
}

macro_rules! extract_field {
    ($field:expr, $msg:expr) => {
        match $field {
            Some(field) => field,
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, $msg))
        }
    }
}

impl Response {
    ///Creates new instance of Response by parsing raw string with it.
    pub fn from_str(msg: &str) -> io::Result<Self> {
        let mut split_msg = msg.splitn(2, ' ');

        let command = extract_field!(split_msg.next(), "VNDB sent empty response.");

        match command {
            "ok" => Ok(Response::Ok),
            "results" => {
                let data = extract_field!(split_msg.next(), "VNDB sent empty results.");
                Ok(Response::Results(response::Results::from_str(data)?))
            },
            "dbstats" => {
                let data = extract_field!(split_msg.next(), "VNDB sent empty dbstats.");
                Ok(Response::DBstats(serde_json::from_str(data)?))
            },
            "error" => {
                let data = extract_field!(split_msg.next(), "VNDB sent empty error.");
                Ok(Response::Error(response::VndbError::from_str(data)?))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unexpected command='{}'", command)))
        }

    }
}
