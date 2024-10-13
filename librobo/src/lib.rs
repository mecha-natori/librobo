//! 環境非依存のロボット開発補助ライブラリ。
//!
//! このライブラリはロボット開発を補助するいくつかのユーティリティが含まれています。

#![allow(unused)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "controller")]
pub mod controller;

#[cfg(feature = "encoder")]
pub mod encoder;

pub mod math;

#[cfg(feature = "servo")]
pub mod servo;

#[cfg(feature = "steering")]
pub mod steering;

pub(crate) mod util;

#[cfg(feature = "bind-c")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
