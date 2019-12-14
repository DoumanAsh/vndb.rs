//!Kawaii VNDB API
//!
//!## Features
//!
//!- `tokio-on` - Enables [tokio](https://tokio.rs/) implementation together with rustls.
//!- `rustls-on` - Enables TLS implementation, using rustls
//!
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

mod utils;
pub mod protocol;
pub mod client;
