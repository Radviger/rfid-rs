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
