//! ステアリング補助モジュール
#![cfg(feature = "steering")]

#[cfg(feature = "controller")]
use crate::controller::NormalizedSticks;
use heapless::Vec;
use num::Complex;

pub mod crawler;

pub mod quad_mechanum;

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
#[cfg(feature = "controller")]
pub trait ISteeringFromSticks<const N: usize> {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        sticks: NormalizedSticks
    ) -> Vec<i16, N>;
}

pub use robo_nostd_macro::ISteeringFromSticks;
