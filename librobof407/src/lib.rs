#![no_std]
#![feature(error_in_core)]

extern crate alloc;

pub use librobo_common::*;

pub mod motor_driver;

#[cfg(test)]
mod test;
