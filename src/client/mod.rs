//! Client implementations

///VNDB Host
pub const API_HOST: &'static str = "api.vndb.org";
///VNDB plain TCP port
pub const API_PORT: u16 = 19534;
///VNDB SSL port
pub const API_SSL_PORT: u16 = 19535;

#[cfg(feature = "tokio-on")]
pub mod tokio;
