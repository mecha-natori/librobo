use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{Display, Formatter}
};

pub mod crawler;

pub mod quad_omni;

#[derive(Clone, Debug)]
pub struct UnsupportedError;

impl Display for UnsupportedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "unsupported operation")
    }
}

impl Error for UnsupportedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct InvalidInputError;

impl Display for InvalidInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid stick input")
    }
}

impl Error for InvalidInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait Steering {
    fn init(&mut self) {}
    fn update(&mut self, index: usize, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
    fn polar(&mut self, r: (f32, f32), theta: (f32, f32)) -> Result<(), Box<dyn Error + 'static>>;
    fn forward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
    fn backward(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
    fn left(&mut self, _speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        Err(Box::new(UnsupportedError))
    }
    fn right(&mut self, _speed: i32) -> Result<(), Box<dyn Error + 'static>> {
        Err(Box::new(UnsupportedError))
    }
    fn turn_left(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
    fn turn_right(&mut self, speed: i32) -> Result<(), Box<dyn Error + 'static>>;
}
