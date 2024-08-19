//! クローラーモジュール
#![cfg(feature = "steering-crawler")]

use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use crate::controller::NormalizedSticks;
use num::Complex;

/// クローラー
#[derive(Clone, Copy, Debug, Eq, ISteeringFromSticks, Ord, PartialEq, PartialOrd)]
pub struct Crawler;

impl ISteering for Crawler {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Right, Left]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16> {
        let r = steering.max_speed as f32 * -r.im;
        let l = steering.max_speed as f32 * l.im;
        [r as i16, l as i16].to_vec()
    }
}
