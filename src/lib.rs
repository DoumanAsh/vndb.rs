//!Kawaii VNDB API
//!
#[macro_use]
extern crate lazy_static;

extern crate bytes;
#[macro_use(try_ready)]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::convert;
use std::error;
use std::net;
use std::ops;
use std::str;
use std::default;
use std::fmt;
use std::io;

pub mod protocol;
pub mod client;
