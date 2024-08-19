//! 四輪オムニホイールモジュール
#![cfg(feature = "steering-quad-omni")]

use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use crate::controller::NormalizedSticks;
use heapless::Vec;
use num::traits::FloatConst;
use num::Complex;
use num::Float;

const N: usize = 4;

/// 四輪オムニホイール
#[derive(Clone, Copy, Debug, Eq, ISteeringFromSticks, Ord, PartialEq, PartialOrd)]
pub struct QuadOmni;

impl ISteering<4> for QuadOmni {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Front-right, Front-left, Rear-left, Rear-right]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<PIDData>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16, 4> {
        let (l_r, l_theta) = l.to_polar();
        let (r_r, r_theta) = r.to_polar();
        let fr = steering.max_speed as f32 * r_r * (f32::FRAC_PI_4() + r_theta).cos();
        let fl = steering.max_speed as f32 * l_r * (f32::FRAC_PI_4() - l_theta).cos();
        let rl = steering.max_speed as f32 * l_r * (3f32 * f32::FRAC_PI_4() - l_theta).cos();
        let rr = steering.max_speed as f32 * r_r * (3f32 * f32::FRAC_PI_4() + r_theta).cos();
        Vec::from_slice(&[fr as i16, fl as i16, rl as i16, rr as i16]).unwrap()
    }
}
