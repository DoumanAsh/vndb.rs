//!Kawaii VNDB API
//!
//!## Features
//!
//!`tokio` - Enables tokio integration, such as codecs implementation
//!
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

pub mod protocol;
#[cfg(feature = "tokio-on")]
pub mod codec;
pub mod client;
