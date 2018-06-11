//!VNDB API Clients
//!
//!
//!# Available clients
//!
//!- `tokio` - Simple tokio based client. Requires to enable `tokio-client` feature

///VNDB Host
pub const API_HOST: &'static str = "api.vndb.org";
///VNDB SSL port
pub const API_SSL_PORT: u16 = 19535;

#[cfg(feature = "tokio-client")]
pub mod tokio;
