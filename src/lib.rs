//!Kawaii VNDB API
//!
//!## Features
//!
//!- `tokio-on` - Enables [tokio](https://tokio.rs/) implementation.
//!- `rustls-on` - Enables TLS implementation, using rustls
//!
//!## TLS client
//!
//!Due to bad default choice of underlying crypto library, `rustls` is included with `default-features = false` which essentially makes it unusable until user provides feature.

#![warn(missing_docs)]
#![allow(clippy::style)]

mod utils;
pub mod protocol;
pub mod client;
