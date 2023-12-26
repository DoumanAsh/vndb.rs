//! Client implementations

use std::io;

///VNDB Host
pub const API_HOST: &'static str = "api.vndb.org";
///VNDB plain TCP port
pub const API_PORT: u16 = 19534;
///VNDB SSL port
pub const API_SSL_PORT: u16 = 19535;

fn parse_response(buf: &[u8]) -> io::Result<Option<crate::protocol::Response>> {
    let buf = match buf.split_last() {
        Some((0x04, buf)) => buf,
        _ => return Ok(None), //incomplete read, connection is reset most likely
    };

    let msg = match core::str::from_utf8(&buf) {
        Ok(msg) => msg,
        Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
    };

    match crate::protocol::Response::from_str(msg) {
        Ok(msg) => Ok(Some(msg)),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
    }
}

#[cfg(feature = "rustls-on")]
fn get_rustls_config() -> (rustls::pki_types::ServerName<'static>, std::sync::Arc<rustls::ClientConfig>) {
    use core::mem::MaybeUninit;
    use core::convert::TryInto;
    use std::sync::Arc;

    static mut SERVER_NAME: MaybeUninit<rustls::pki_types::ServerName<'static>> = MaybeUninit::uninit();
    static mut CFG: MaybeUninit<Arc<rustls::ClientConfig>> = MaybeUninit::uninit();
    static CFG_ONCE: std::sync::Once = std::sync::Once::new();

    CFG_ONCE.call_once(|| {
        let mut certs = rustls::RootCertStore::empty();
        certs.extend(webpki_roots::TLS_SERVER_ROOTS.into_iter().cloned());
        let config = rustls::ClientConfig::builder().with_root_certificates(certs)
                                                    .with_no_client_auth();
        unsafe {
            CFG.as_mut_ptr().write(Arc::new(config));
        }

        let server = match API_HOST.try_into() {
            Ok(server) => server,
            Err(_) => unreachable!()
        };

        unsafe {
            SERVER_NAME.as_mut_ptr().write(server)
        }
    });

    unsafe {
        ((*SERVER_NAME.as_ptr()).clone(), (*(CFG.as_ptr())).clone())
    }
}

pub mod simple;
///Alias to simple std based client
pub type Simple<IO> = simple::Client<IO>;

#[cfg(feature = "tokio-on")]
pub mod tokio;
#[cfg(feature = "tokio-on")]
///Alias to tokio based client
pub type Tokio<IO> = tokio::Client<IO>;
