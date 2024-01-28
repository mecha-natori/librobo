use core::f32::consts::PI;
use either::Either;
use embedded_hal::digital::v2::InputPin;
use rotary_encoder_hal::{Direction, Rotary};

#[derive(Debug)]
pub struct Encoder<A, B>
where
    A: InputPin,
    B: InputPin
{
    enc: Rotary<A, B>,
    rot: i32,
    delta_rot: i32,
    cpr: u16
}

impl<A, B> Encoder<A, B>
where
    A: InputPin,
    B: InputPin
{
    pub fn new(enc: Rotary<A, B>, ppr: u16) -> Self {
        Self {
            enc,
            rot: 0,
            delta_rot: 0,
            cpr: ppr * 4
        }
    }

    pub fn get_rot(&self) -> i32 {
        self.rot
    }

    pub fn get_delta_rot(&mut self) -> i32 {
        let current = self.delta_rot;
        self.delta_rot = 0;
        current
    }

    pub fn get_deg(&self) -> f32 {
        (self.rot / self.cpr as i32) as f32 * 360f32
    }

    pub fn get_delta_deg(&mut self) -> f32 {
        (self.get_delta_rot() / self.cpr as i32) as f32 * 360f32
    }

    pub fn get_rad(&self) -> f32 {
        (self.rot / self.cpr as i32) as f32 * PI * 2f32
    }

    pub fn get_delta_rad(&mut self) -> f32 {
        (self.get_delta_rot() / self.cpr as i32) as f32 * PI * 2f32
    }

    pub fn get_rev(&self) -> i32 {
        self.rot / self.cpr as i32
    }

    pub fn get_delta_rev(&mut self) -> i32 {
        self.get_delta_rot() / self.cpr as i32
    }

    pub fn update(&mut self) -> Result<(), Either<A::Error, B::Error>> {
        match self.enc.update()? {
            Direction::Clockwise => {
                self.rot += 1;
                self.delta_rot += 1;
            }
            Direction::CounterClockwise => {
                self.rot -= 1;
                self.delta_rot -= 1;
            }
            Direction::None => {}
        }
        Ok(())
    }
}
