//!VNDB messages serialization/de-serialization.

use core::{str, fmt, convert};

pub mod request;
pub mod response;

#[derive(Clone)]
///VNDB Request
///
///On error returns [Response::Error](response/Struct.VndbError.html).
pub enum Request<'a> {
    ///Login request.
    ///
    ///On success returns `Response::Ok`
    Login(request::Login<'a>),
    ///Get request.
    ///
    ///On success returns [Response::Results](response/Struct.Results.html)
    Get(request::Get<'a>),
    ///VNDB statistic request.
    ///
    ///On success returns [Response::DBstats](response/Struct.DBstats.html)
    DBstats
}

impl<'a> convert::From<request::Login<'a>> for Request<'a> {
    fn from(login: request::Login<'a>) -> Self {
        Request::Login(login)
    }
}

impl<'a> convert::From<request::Get<'a>> for Request<'a> {
    fn from(get: request::Get<'a>) -> Self {
        Request::Get(get)
    }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Request::Login(ref login) => write!(f, "{}\x04", login),
            &Request::Get(ref get) => write!(f, "{}\x04", get),
            &Request::DBstats => write!(f, "dbstats\x04")
        }
    }
}

#[derive(Debug, Clone)]
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

impl convert::From<response::Results> for Response {
    fn from(results: response::Results) -> Self {
        Response::Results(results)
    }
}

impl convert::From<response::DBstats> for Response {
    fn from(stats: response::DBstats) -> Self {
        Response::DBstats(stats)
    }
}

impl convert::From<response::VndbError> for Response {
    fn from(error: response::VndbError) -> Self {
        Response::Error(error)
    }
}

impl Response {
    ///Parses response from text message without 0x04 byte.
    pub fn from_str(msg: &str) -> Result<Self, ResponseParseError> {
        let mut split_msg = msg.splitn(2, ' ');

        let command = match split_msg.next() {
            Some(command) => command,
            None => return Err(ResponseParseError::EmptyResponse),
        };

        match command {
            "ok" => Ok(Response::Ok),
            "results" => match split_msg.next() {
                Some(results) => match response::Results::from_str(results) {
                    Ok(results) => Ok(Response::Results(results)),
                    Err(error) => Err(ResponseParseError::InvalidResults(error)),
                },
                None => Err(ResponseParseError::EmptyResults),
            },
            "dbstats" => match split_msg.next() {
                Some(dbstats) => match serde_json::from_str(dbstats) {
                    Ok(dbstats) => Ok(Response::DBstats(dbstats)),
                    Err(error) => Err(ResponseParseError::InvalidDbStats(error)),
                },
                None => Err(ResponseParseError::EmptyDbStats),
            },
            "error" => match split_msg.next() {
                Some(error) => match serde_json::from_str(error) {
                    Ok(error) => Ok(Response::Error(error)),
                    Err(error) => Err(ResponseParseError::InvalidError(error)),
                },
                None => Err(ResponseParseError::EmptyDbStats),
            }
            _ => Err(ResponseParseError::UnknownComamnd),
        }
    }
}

#[derive(Debug)]
///Result of Response parser
pub enum ResponseParseError {
    ///Response is empty
    EmptyResponse,
    ///Results is without payload
    EmptyResults,
    ///DBstats is without payload
    EmptyDbStats,
    ///Error is without payload
    EmptyError,
    ///Invalid Results payload.
    InvalidResults(serde_json::Error),
    ///Invalid DBstats payload.
    InvalidDbStats(serde_json::Error),
    ///Invalid Error payload.
    InvalidError(serde_json::Error),
    ///Unknown command is specified.
    UnknownComamnd,
}

impl fmt::Display for ResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResponseParseError::EmptyResponse => write!(f, "VNDB sent empty response."),
            &ResponseParseError::EmptyResults => write!(f, "VNDB sent Results with no payload."),
            &ResponseParseError::EmptyDbStats => write!(f, "VNDB sent DBstats with no payload."),
            &ResponseParseError::EmptyError => write!(f, "VNDB sent Error with no payload."),
            &ResponseParseError::InvalidResults(ref error) => write!(f, "VNDB sent invalid JSON in Results: {}", error),
            &ResponseParseError::InvalidDbStats(ref error) => write!(f, "VNDB sent invalid JSON in DBstats: {}", error),
            &ResponseParseError::InvalidError(ref error) => write!(f, "VNDB sent invalid JSON in Error: {}", error),
            &ResponseParseError::UnknownComamnd => write!(f, "VNDB sent unknown command"),
        }
    }
}

impl std::error::Error for ResponseParseError {}
