//!Kawaii VNDB API
//!
#![warn(missing_docs)]

#[cfg(feature = "tokio-client")]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod protocol;
pub mod client;
