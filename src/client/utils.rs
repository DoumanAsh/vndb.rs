extern crate tokio_tls;
extern crate native_tls;

use ::io;

pub trait IoError<T> {
    type Ok;

    fn map_io(self) -> Result<Self::Ok, io::Error>;
}

impl<T> IoError<T> for native_tls::Result<T> {
    type Ok = T;

    fn map_io(self) -> Result<Self::Ok, io::Error> {
        self.map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }
}

//pub fn error<E: Into<Box<error::Error+Send+Sync>>>(error: E) -> io::Error {
//    io::Error::new(io::ErrorKind::Other, error)
//}

#[doc(hidden)]
#[macro_export]
macro_rules! map_io {
    () => { |e| io::Error::new(io::ErrorKind::Other, e) }
}
