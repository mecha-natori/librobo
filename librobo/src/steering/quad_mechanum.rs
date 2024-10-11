//! 四輪メカナムホイールモジュール

use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use crate::controller::NormalizedSticks;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "heapless")]
use heapless::Vec;
use num::traits::FloatConst;
use num::Complex;
#[cfg(not(feature = "std"))]
use num::Float;

#[cfg(feature = "bind-c")]
mod ffi;

#[cfg(feature = "heapless")]
const N: usize = 4;

/// 四輪メカナムホイール
#[cfg_attr(feature = "controller", derive(ISteeringFromSticks))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QuadMechanum;

#[cfg(any(feature = "alloc", feature = "std"))]
impl ISteering for QuadMechanum {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Front-right, Front-left, Rear-left, Rear-right]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut Vec<PIDData>>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16> {
        let (l_r, l_theta) = l.to_polar();
        let (r_r, r_theta) = r.to_polar();
        let fr = steering.max_speed as f32 * r_r * (f32::FRAC_PI_4() + r_theta).cos();
        let fl = steering.max_speed as f32 * l_r * (f32::FRAC_PI_4() - l_theta).cos();
        let rl = steering.max_speed as f32 * l_r * (3f32 * f32::FRAC_PI_4() - l_theta).cos();
        let rr = steering.max_speed as f32 * r_r * (3f32 * f32::FRAC_PI_4() + r_theta).cos();
        [fr as i16, fl as i16, rl as i16, rr as i16].to_vec()
    }
}

#[cfg(feature = "heapless")]
impl ISteering<N> for QuadMechanum {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Front-right, Front-left, Rear-left, Rear-right]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut Vec<PIDData, N>>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16, N> {
        let (l_r, l_theta) = l.to_polar();
        let (r_r, r_theta) = r.to_polar();
        let fr = steering.max_speed as f32 * r_r * (f32::FRAC_PI_4() + r_theta).cos();
        let fl = steering.max_speed as f32 * l_r * (f32::FRAC_PI_4() - l_theta).cos();
        let rl = steering.max_speed as f32 * l_r * (3f32 * f32::FRAC_PI_4() - l_theta).cos();
        let rr = steering.max_speed as f32 * r_r * (3f32 * f32::FRAC_PI_4() + r_theta).cos();
        Vec::from_slice(&[fr as i16, fl as i16, rl as i16, rr as i16]).unwrap()
    }
}
