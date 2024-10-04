//! ステアリング補助モジュール

#[cfg(feature = "controller")]
use crate::controller::NormalizedSticks;
#[cfg(not(feature = "std"))]
use heapless::Vec;
use num::Complex;

#[cfg(feature = "steering-crawler")]
pub mod crawler;

#[cfg(feature = "steering-quad-mechanum")]
pub mod quad_mechanum;

#[cfg(feature = "steering-quad-omni")]
pub mod quad_omni;

/// PID制御データ
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct PIDData {
    /// Pゲイン
    pub kp: f32,
    /// Iゲイン
    pub ki: f32,
    /// Dゲイン
    pub kd: f32,
    /// 前回操作量
    pub prev_mv: i16,
    /// 前回累計偏差
    pub prev_e: i16,
    /// 前々回累計偏差
    pub prev_prev_e: i16
}

/// ステアリングデータ
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Steering {
    /// 最高速度 \[rpm]
    pub max_speed: i16
}

/// ステアリングインターフェース
#[cfg(feature = "std")]
pub trait ISteering {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16>;
}

/// ステアリングインターフェース
#[cfg(not(feature = "std"))]
pub trait ISteering<const N: usize> {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16, N>;
}

/// ステアリングインターフェース
#[cfg(all(feature = "controller", feature = "std"))]
pub trait ISteeringFromSticks {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        sticks: NormalizedSticks
    ) -> Vec<i16>;
}

/// ステアリングインターフェース
#[cfg(all(feature = "controller", not(feature = "std")))]
pub trait ISteeringFromSticks<const N: usize> {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        sticks: NormalizedSticks
    ) -> Vec<i16, N>;
}

pub use robo_macro::ISteeringFromSticks;
