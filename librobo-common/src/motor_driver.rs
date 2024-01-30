use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{Display, Formatter}
};
use embedded_hal::digital::v2::InputPin;

#[derive(Clone, Debug)]
pub struct DefaultMethodUnsupportedError;

impl Display for DefaultMethodUnsupportedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "default method is unsupported, please use unique method")
    }
}

impl Error for DefaultMethodUnsupportedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait MotorDriver<A, B>
where
    A: InputPin,
    B: InputPin
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        Err(Box::new(DefaultMethodUnsupportedError))
    }
    fn set_target(&mut self, _speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        Err(Box::new(DefaultMethodUnsupportedError))
    }
    fn get_max_speed(&self) -> i32;
}
