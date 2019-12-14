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
