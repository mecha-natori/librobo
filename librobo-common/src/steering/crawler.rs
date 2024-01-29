use crate::{motor_driver::MotorDriver, steering::Steering};
use alloc::boxed::Box;
use core::{cmp::max, error::Error, f32::consts::PI};
use embedded_hal::digital::v2::InputPin;
#[allow(unused_imports)]
use num_traits::real::Real;

pub struct STCrawler<A, B>
where
    A: InputPin,
    B: InputPin
{
    md: [Box<dyn MotorDriver<A, B>>; 2]
}

impl<A, B> STCrawler<A, B>
where
    A: InputPin,
    B: InputPin
{
    pub fn new(right: Box<dyn MotorDriver<A, B>>, left: Box<dyn MotorDriver<A, B>>) -> Self {
        Self { md: [right, left] }
    }
}

impl<A, B> Steering for STCrawler<A, B>
where
    A: InputPin,
    B: InputPin
{
    fn update(&mut self, index: usize, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[index].set_target(speed)?;
        Ok(())
    }

    fn polar(&mut self, r: f32, theta: (f32, f32)) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target((theta.0.sin() * r * self.md[0].get_max_speed() as f32) as i32)?;
        self.md[1].set_target((theta.1.sin() * r * self.md[1].get_max_speed() as f32) as i32)?;
        Ok(())
    }

    fn forward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0]
            .set_target((PI / 4f32 * max(speed.abs(), self.md[0].get_max_speed()) as f32) as i32)?;
        self.md[1]
            .set_target((PI / 4f32 * max(speed.abs(), self.md[1].get_max_speed()) as f32) as i32)?;
        Ok(())
    }

    fn backward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(
            -(PI / 4f32 * max(speed.abs(), self.md[0].get_max_speed()) as f32) as i32
        )?;
        self.md[1].set_target(
            -(PI / 4f32 * max(speed.abs(), self.md[1].get_max_speed()) as f32) as i32
        )?;
        Ok(())
    }

    fn turn_left(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0]
            .set_target((PI / 4f32 * max(speed.abs(), self.md[0].get_max_speed()) as f32) as i32)?;
        self.md[1].set_target(
            -(PI / 4f32 * max(speed.abs(), self.md[1].get_max_speed()) as f32) as i32
        )?;
        Ok(())
    }

    fn turn_right(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        self.md[0].set_target(
            -(PI / 4f32 * max(speed.abs(), self.md[0].get_max_speed()) as f32) as i32
        )?;
        self.md[1]
            .set_target((PI / 4f32 * max(speed.abs(), self.md[1].get_max_speed()) as f32) as i32)?;
        Ok(())
    }
}