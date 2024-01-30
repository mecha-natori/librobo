use alloc::boxed::Box;
use core::{error::Error, marker::PhantomData};
use librobo_common::{encoder::Encoder, motor_driver::MotorDriver};
use stm32f4xx_hal::{
    hal_02::digital::v2::{InputPin, OutputPin},
    pac::{TIM1, TIM2, TIM3, TIM4, TIM8},
    prelude::*,
    rcc::Clocks,
    timer::{Channel, Pins, PwmHz}
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
    dir: Direction<D>,
    _t: PhantomData<T>
}

impl<A, B, D> PwmMD<A, B, TIM1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(max_speed: i32, enc: Encoder<A, B>, dir: D) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            dir,
            _t: PhantomData
        }
    }

    pub fn init<P>(&mut self, pwm: TIM1, pins: P, clocks: &Clocks) -> PwmHz<TIM1, P>
    where
        P: Pins<TIM1>
    {
        let mut pwm = pwm.pwm_hz(pins, 1.kHz(), clocks);
        pwm.enable(Channel::C1);
        pwm
    }

    pub fn set_target<P>(
        &mut self,
        speed: i32,
        pwm: &mut PwmHz<TIM1, P>,
        ch: Channel
    ) -> Result<(), Box<dyn Error + 'static>>
    where
        P: Pins<TIM1>
    {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        pwm.set_duty(ch, duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, TIM1, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, TIM2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(max_speed: i32, enc: Encoder<A, B>, dir: D) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            dir,
            _t: PhantomData
        }
    }

    pub fn init<P>(&mut self, pwm: TIM2, pins: P, clocks: &Clocks) -> PwmHz<TIM2, P>
    where
        P: Pins<TIM2>
    {
        let mut pwm = pwm.pwm_hz(pins, 1.kHz(), clocks);
        pwm.enable(Channel::C1);
        pwm
    }

    pub fn set_target<P>(
        &mut self,
        speed: i32,
        pwm: &mut PwmHz<TIM2, P>,
        ch: Channel
    ) -> Result<(), Box<dyn Error + 'static>>
    where
        P: Pins<TIM2>
    {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        pwm.set_duty(ch, duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, TIM2, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, TIM3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(max_speed: i32, enc: Encoder<A, B>, dir: D) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            dir,
            _t: PhantomData
        }
    }

    pub fn init<P>(&mut self, pwm: TIM3, pins: P, clocks: &Clocks) -> PwmHz<TIM3, P>
    where
        P: Pins<TIM3>
    {
        let mut pwm = pwm.pwm_hz(pins, 1.kHz(), clocks);
        pwm.enable(Channel::C1);
        pwm
    }

    pub fn set_target<P>(
        &mut self,
        speed: i32,
        pwm: &mut PwmHz<TIM3, P>,
        ch: Channel
    ) -> Result<(), Box<dyn Error + 'static>>
    where
        P: Pins<TIM3>
    {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        pwm.set_duty(ch, duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, TIM3, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, TIM4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(max_speed: i32, enc: Encoder<A, B>, dir: D) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            dir,
            _t: PhantomData
        }
    }

    pub fn init<P>(&mut self, pwm: TIM4, pins: P, clocks: &Clocks) -> PwmHz<TIM4, P>
    where
        P: Pins<TIM4>
    {
        let mut pwm = pwm.pwm_hz(pins, 1.kHz(), clocks);
        pwm.enable(Channel::C1);
        pwm
    }

    pub fn set_target<P>(
        &mut self,
        speed: i32,
        pwm: &mut PwmHz<TIM4, P>,
        ch: Channel
    ) -> Result<(), Box<dyn Error + 'static>>
    where
        P: Pins<TIM4>
    {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        pwm.set_duty(ch, duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, TIM4, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}

impl<A, B, D> PwmMD<A, B, TIM8, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    pub fn new(max_speed: i32, enc: Encoder<A, B>, dir: D) -> Self {
        let dir = Direction::new(dir);
        Self {
            max_speed,
            enc,
            dir,
            _t: PhantomData
        }
    }

    pub fn init<P>(&mut self, pwm: TIM8, pins: P, clocks: &Clocks) -> PwmHz<TIM8, P>
    where
        P: Pins<TIM8>
    {
        let mut pwm = pwm.pwm_hz(pins, 1.kHz(), clocks);
        pwm.enable(Channel::C1);
        pwm
    }

    pub fn set_target<P>(
        &mut self,
        speed: i32,
        pwm: &mut PwmHz<TIM8, P>,
        ch: Channel
    ) -> Result<(), Box<dyn Error + 'static>>
    where
        P: Pins<TIM8>
    {
        let dir = speed < 0;
        let speed = speed.unsigned_abs();
        let now = (self.enc.get_delta_rot() * 1000i32).unsigned_abs();
        let kn = 4f32;
        let gain_c1 = kn + 1f32 / kn;
        let gain_c2 = 1f32 / kn;
        let duty = (speed as f32 * gain_c1 - now as f32 * gain_c2) as u16;
        pwm.set_duty(ch, duty);
        if dir {
            self.dir.left()?;
        } else {
            self.dir.right()?;
        }
        Ok(())
    }
}

impl<A, B, D> MotorDriver<A, B> for PwmMD<A, B, TIM8, D>
where
    A: InputPin,
    B: InputPin,
    D: OutputPin<Error = Box<dyn Error + 'static>>
{
    fn get_max_speed(&self) -> i32 {
        self.max_speed
    }
}
