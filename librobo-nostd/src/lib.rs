//! 環境非依存のロボット開発補助ライブラリ。
//! このライブラリはロボット開発を補助するいくつかのユーティリティが含まれています。
//! このライブラリはstdクレートを使用せず、代わりにcoreクレートを使用します。

#![allow(unused)]
#![no_std]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

pub mod controller;

pub mod encoder;

pub mod math;

pub mod steering;
