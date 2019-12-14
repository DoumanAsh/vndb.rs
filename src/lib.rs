//!Kawaii VNDB API
//!
//!## Features
//!
//!`rustls-on` - Enables TLS implementation, using rustls
//!
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

pub mod protocol;
pub mod client;
