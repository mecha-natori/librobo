//! 他ライブラリとの連携クレート。

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

#[cfg(feature = "esp32-ds4-driver")]
pub mod esp32_ds4_driver;
