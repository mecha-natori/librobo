//! ステアリング補助モジュール

use crate::controller::NormalizedSticks;
use crate::debug_log;
use crate::trace_log;
use core::error::Error;
use core::fmt::Display;
use core::fmt::Formatter;
#[cfg(feature = "heapless")]
use heapless::Vec as HVec;
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

/// パラメータ不足エラー
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingParameterError;

impl Display for MissingParameterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Missing some PID parameter(s).")
    }
}

impl Error for MissingParameterError {
}

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
        debug_log!(target: "librobo/steering", "build PIDData from: {:?}", &self);
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

#[cfg(feature = "controller")]
impl<T, const N: usize> ISteeringFromSticks<N> for T
where
    T: ISteering<N>
{
    /// 速度を計算する。 \[rpm]
    #[inline]
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut [PIDData; N]>,
        sticks: NormalizedSticks
    ) -> [i16; N] {
        <Self as ISteering<N>>::calc_speed(
            steering,
            pid_data,
            Complex::new(sticks.l[0], sticks.l[1]),
            Complex::new(sticks.r[0], sticks.r[1])
        )
    }
}

/// PIDデータに基づいて目標値を加工する。
pub fn process_pid_data(pid_data: &mut PIDData, target: f32) -> f32 {
    debug_log!(target: "librobo/steering", "process target {} with PID data: {:?}", target, pid_data);
    let e = target - pid_data.now_out;
    let de = (e - pid_data.prev_e) / pid_data.t;
    let ie = pid_data.prev_ie + (e + pid_data.prev_e) * pid_data.t / 2f32;
    trace_log!(target: "librobo/steering", "e: {}, ie: {}, de: {}", e, ie, de);
    pid_data.prev_e = e;
    pid_data.prev_ie = ie;
    let processed = pid_data.kp * e + pid_data.ki * ie + pid_data.kd * de;
    debug_log!(target: "librobo/steering", "processed target {}", processed);
    processed
}
