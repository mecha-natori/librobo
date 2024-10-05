//! ステアリング補助モジュール

#[cfg(feature = "controller")]
use crate::controller::NormalizedSticks;
#[cfg(not(feature = "std"))]
use heapless::Vec;
use num::Complex;

#[cfg(feature = "steering-crawler")]
pub mod crawler;

#[cfg(feature = "bind-c")]
mod ffi;

#[cfg(feature = "steering-quad-mechanum")]
pub mod quad_mechanum;

#[cfg(feature = "steering-quad-omni")]
pub mod quad_omni;

/// PID制御データ
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct PIDData {
    /// Pゲイン
    pub kp: f32,
    /// Iゲイン
    pub ki: f32,
    /// Dゲイン
    pub kd: f32,
    /// 前回累計偏差
    pub prev_e: f32,
    /// 前回累計偏差積分
    pub prev_ie: f32,
    /// 現在出力
    pub now_out: f32,
    /// 制御周期
    pub t: f32
}

/// ステアリングデータ
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
        pid_data: Option<&mut Vec<PIDData>>,
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
        pid_data: Option<&mut Vec<PIDData, N>>,
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
        pid_data: Option<&mut Vec<PIDData>>,
        sticks: NormalizedSticks
    ) -> Vec<i16>;
}

/// ステアリングインターフェース
#[cfg(all(feature = "controller", not(feature = "std")))]
pub trait ISteeringFromSticks<const N: usize> {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut Vec<PIDData, N>>,
        sticks: NormalizedSticks
    ) -> Vec<i16, N>;
}

#[cfg(feature = "controller")]
pub use robo_macro::ISteeringFromSticks;

/// PIDデータに基づいて目標値を加工する。
pub fn process_pid_data(pid_data: &mut PIDData, target: f32) -> f32 {
    let e = target - pid_data.now_out;
    let de = (e - pid_data.prev_e) / pid_data.t;
    let ie = pid_data.prev_ie + (e + pid_data.prev_e) * pid_data.t / 2f32;
    pid_data.prev_e = e;
    pid_data.prev_ie = ie;
    pid_data.kp * e + pid_data.ki * ie + pid_data.kd * de
}
