use crate::{
    motor_driver::MotorDriver,
    steering::{InvalidInputError, Steering}
};
use alloc::boxed::Box;
use core::{cmp::max, error::Error, f32::consts::PI};
use embedded_hal::digital::v2::InputPin;
use num_traits::real::Real;

pub struct STQuadOmni<A, B>
where
    A: InputPin,
    B: InputPin
{
    md: [Box<dyn MotorDriver<A, B>>; 4]
}

impl<A, B> STQuadOmni<A, B>
where
    A: InputPin,
    B: InputPin
{
    pub fn new(
        front_right: Box<dyn MotorDriver<A, B>>,
        front_left: Box<dyn MotorDriver<A, B>>,
        rear_left: Box<dyn MotorDriver<A, B>>,
        rear_right: Box<dyn MotorDriver<A, B>>
    ) -> Self {
        Self {
            md: [front_right, front_left, rear_left, rear_right]
        }
    }
}

impl<A, B> Steering for STQuadOmni<A, B>
where
    A: InputPin,
    B: InputPin
{
    fn update(&mut self, index: usize, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[index].set_target(speed)?;
        Ok(())
    }

    fn polar(&mut self, r: (f32, f32), theta: (f32, f32)) -> Result<(), Box<dyn Error + 'static>> {
        if !(0f32..=1f32).contains(&r.0) {
            return Err(Box::new(InvalidInputError));
        }
        self.md[0].set_target(
            (r.0 * self.md[0].get_max_speed() as f32 * (theta.0 - PI / 4f32).cos()) as i32
        )?;
        self.md[1].set_target(
            (r.0 * self.md[1].get_max_speed() as f32 * (theta.0 + PI / 4f32).cos()) as i32
        )?;
        self.md[2].set_target(
            (r.0 * self.md[2].get_max_speed() as f32 * (theta.0 + 3f32 * PI / 4f32).cos()) as i32
        )?;
        self.md[3].set_target(
            (r.0 * self.md[3].get_max_speed() as f32 * (theta.0 - 3f32 * PI / 4f32).cos()) as i32
        )?;
        Ok(())
    }

    fn forward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(-max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(-max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }

    fn backward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(-max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(-max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }

    fn left(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(-max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(-max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }

    fn right(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(-max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(-max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }

    fn turn_left(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }

    fn turn_right(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(-max(speed.abs(), self.md[0].get_max_speed()))?;
        self.md[1].set_target(-max(speed.abs(), self.md[1].get_max_speed()))?;
        self.md[2].set_target(-max(speed.abs(), self.md[2].get_max_speed()))?;
        self.md[3].set_target(-max(speed.abs(), self.md[3].get_max_speed()))?;
        Ok(())
    }
}
