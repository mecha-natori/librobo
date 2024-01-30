use alloc::boxed::Box;
use core::error::Error;
use librobo_common::{encoder::Encoder, motor_driver::MotorDriver};
use stm32f3xx_hal::{
    hal::{
        digital::v2::{InputPin, OutputPin},
        PwmPin
    },
    pwm::{
        PwmChannel, Tim1Ch1, Tim1Ch2, Tim1Ch3, Tim1Ch4, Tim2Ch1, Tim2Ch2, Tim2Ch3, Tim2Ch4,
        Tim3Ch1, Tim3Ch2, Tim3Ch3, Tim3Ch4, Tim4Ch1, Tim4Ch2, Tim4Ch3, Tim4Ch4, Tim8Ch1, Tim8Ch2,
        Tim8Ch3, Tim8Ch4, WithPins
    }
};

#[derive(Debug)]
struct Direction<A>
where
    A: OutputPin<Error = Box<dyn Error + 'static>>
{
    pin: A
}

impl<A> Direction<A>
where
    A: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(pin: A) -> Self {
        Self { pin }
    }

    pub fn left(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pin.set_high()?;
        Ok(())
    }

    pub fn right(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pin.set_low()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PwmMD<A, B, T, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    max_speed: i32,
    enc: Encoder<A, B>,
    pwm: PwmChannel<T, WithPins>,
    dir: Direction<D>
}

impl<A, B, D> PwmMD<A, B, Tim1Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim1Ch1, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim1Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim1Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim1Ch2, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim1Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim1Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim1Ch3, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim1Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim1Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim1Ch4, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim1Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim2Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim2Ch1, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim2Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u32;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim2Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim2Ch2, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim2Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u32;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim2Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim2Ch3, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim2Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u32;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim2Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim2Ch4, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim2Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u32;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim3Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim3Ch1, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim3Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim3Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim3Ch2, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim3Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim3Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim3Ch3, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim3Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim3Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim3Ch4, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim3Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim4Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim4Ch1, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim4Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim4Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim4Ch2, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim4Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim4Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim4Ch3, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim4Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim4Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim4Ch4, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim4Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim8Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim8Ch1, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim8Ch1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim8Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim8Ch2, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim8Ch2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim8Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim8Ch3, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim8Ch3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, Tim8Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(
        max_speed: i32,
        enc: Encoder<A, B>,
        pwm: PwmChannel<Tim8Ch4, WithPins>,
        dir: D
    ) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            pwm,
            dir
        }
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, Tim8Ch4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn init(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        self.pwm.enable();
        Ok(())
    }

    fn set_target(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        self.pwm.set_duty(duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }

    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}
