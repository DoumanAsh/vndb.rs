//!Kawaii VNDB API
//!

extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::ops;
use std::str;
use std::default;
use std::fmt;
use std::io;

pub mod protocol;
