//! ステアリング補助モジュール

use crate::controller::NormalizedSticks;
use core::error::Error;
use core::fmt::Display;
use core::fmt::Formatter;
#[cfg(feature = "heapless")]
use heapless::Vec as HVec;
use num::Complex;
#[cfg(feature = "controller")]
pub use robo_macro::ISteeringFromSticks;

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

/// パラメータ不足エラー
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingParameterError;

impl Display for MissingParameterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Missing some PID parameter(s).")
    }
}

impl Error for MissingParameterError {}

/// PID制御データのビルダー
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PIDDataBuilder {
    kp: Option<f32>,
    ki: Option<f32>,
    kd: Option<f32>,
    t: Option<f32>
}

impl PIDDataBuilder {
    /// PID制御データを作成する。
    ///
    /// # Error
    /// 制御周期の指定がされていない場合、[MissingParameterError]が返される。
    pub fn build(self) -> Result<PIDData, MissingParameterError> {
        let Some(t) = self.t else {
            return Err(MissingParameterError);
        };
        Ok(PIDData {
            kp: self.kp.unwrap_or(0f32),
            ki: self.ki.unwrap_or(0f32),
            kd: self.kd.unwrap_or(0f32),
            prev_e: 0f32,
            prev_ie: 0f32,
            now_out: 0f32,
            t
        })
    }

    /// Pゲインを設定する。
    pub fn kp(mut self, kp: f32) -> Self {
        self.kp = Some(kp);
        self
    }

    /// Iゲインを設定する。
    pub fn ki(mut self, ki: f32) -> Self {
        self.ki = Some(ki);
        self
    }

    /// Dゲインを設定する。
    pub fn kd(mut self, kd: f32) -> Self {
        self.kd = Some(kd);
        self
    }

    /// 制御周期を設定する。 \[sec]
    pub fn t(mut self, t: f32) -> Self {
        self.t = Some(t);
        self
    }
}

/// ステアリングデータ
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
        pid_data: Option<&mut [PIDData; N]>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> [i16; N];
}

/// ステアリングインターフェース
#[cfg(feature = "controller")]
pub trait ISteeringFromSticks<const N: usize> {
    /// 速度を計算する。 \[rpm]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut [PIDData; N]>,
        sticks: NormalizedSticks
    ) -> [i16; N];
}

/// PIDデータに基づいて目標値を加工する。
pub fn process_pid_data(pid_data: &mut PIDData, target: f32) -> f32 {
    let e = target - pid_data.now_out;
    let de = (e - pid_data.prev_e) / pid_data.t;
    let ie = pid_data.prev_ie + (e + pid_data.prev_e) * pid_data.t / 2f32;
    pid_data.prev_e = e;
    pid_data.prev_ie = ie;
    pid_data.kp * e + pid_data.ki * ie + pid_data.kd * de
}
