use embedded_hal::digital::v2::{OutputPin, PinState};

pub struct DummyNSS;

impl OutputPin for DummyNSS {
    type Error = ();
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_state(&mut self, _state: PinState) -> Result<(), Self::Error> {
        Ok(())
    }
}

mod sealed {
    /// A trait that can be implemented to limit implementations to this crate.
    /// See the [Sealed traits pattern](https://rust-lang.github.io/api-guidelines/future-proofing.html)
    /// for more info.
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;
