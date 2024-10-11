//! クローラーモジュール

use super::process_pid_data;
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
use num::Complex;

#[cfg(feature = "bind-c")]
mod ffi;

#[cfg(feature = "heapless")]
const N: usize = 2;

/// クローラー
#[cfg_attr(feature = "controller", derive(ISteeringFromSticks))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Crawler;

#[cfg(any(feature = "alloc", feature = "std"))]
impl ISteering for Crawler {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Right, Left]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut Vec<PIDData>>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16> {
        let r = steering.max_speed as f32 * -r.im;
        let l = steering.max_speed as f32 * l.im;
        if let Some(mut pid_data) = pid_data {
            let r = process_pid_data(&mut pid_data[0], r);
            let l = process_pid_data(&mut pid_data[1], l);
            [r as i16, l as i16].to_vec()
        } else {
            [r as i16, l as i16].to_vec()
        }
    }
}

#[cfg(feature = "heapless")]
impl ISteering<N> for Crawler {
    /// 速度を計算する。 \[rpm]
    ///
    /// 戻り値は\[Right, Left]。
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut Vec<PIDData, N>>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> Vec<i16, N> {
        let r = steering.max_speed as f32 * -r.im;
        let l = steering.max_speed as f32 * l.im;
        if let Some(pid_data) = pid_data {
            let r = process_pid_data(&mut pid_data[0], r);
            let l = process_pid_data(&mut pid_data[1], l);
            Vec::from_slice(&[r as i16, l as i16]).unwrap()
        } else {
            Vec::from_slice(&[r as i16, l as i16]).unwrap()
        }
    }
}
