//!Kawaii VNDB API

#[macro_use]
extern crate error_chain;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate native_tls;

use std::default;
use std::fmt;
use std::net;
use std::io;

pub mod message;
pub mod client;
