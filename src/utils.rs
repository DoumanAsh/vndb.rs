pub(crate) trait AsPin {
    ///Gets `Pin` out of self.
    fn as_pin(&mut self) -> core::pin::Pin<&'_ mut Self>;
}

impl<T> AsPin for T {
    #[inline(always)]
    fn as_pin(&mut self) -> core::pin::Pin<&'_ mut Self> {
        unsafe {
            core::pin::Pin::new_unchecked(self)
        }
    }
}

pub fn serde_from_str<'de, T: core::str::FromStr, D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<T, D::Error> where T::Err: core::fmt::Display {
    use serde::Deserialize;

    let s = <&str>::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
