use alloc::boxed::Box;
use core::error::Error;
use embedded_hal::digital::v2::InputPin;

pub trait MotorDriver<A, B>
where
    A: InputPin,
    B: InputPin
{
    fn init(&mut self);
    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
    fn get_max_speed(&self) -> i32;
}
