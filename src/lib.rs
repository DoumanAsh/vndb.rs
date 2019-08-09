//!Kawaii VNDB API
//!
//!## Features
//!
//!`tokio` - Enables tokio integration, such as codecs implementation
//!
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(feature = "tokio-on", feature(async_await))]

pub mod protocol;
#[cfg(feature = "tokio-on")]
pub mod codec;
pub mod client;
