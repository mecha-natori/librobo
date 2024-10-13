//! クローラーモジュール

use super::process_pid_data;
use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use num::Complex;

#[cfg(feature = "bind-c")]
mod ffi;

const N: usize = 2;

/// クローラー
#[cfg_attr(feature = "controller", derive(ISteeringFromSticks))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Crawler;

impl ISteering<N> for Crawler {
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut [PIDData; N]>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> [i16; N] {
        let r = steering.max_speed as f32 * -r.im;
        let l = steering.max_speed as f32 * l.im;
        if let Some(mut pid_data) = pid_data {
            let r = process_pid_data(&mut pid_data[0], r);
            let l = process_pid_data(&mut pid_data[1], l);
            [r as i16, l as i16]
        } else {
            [r as i16, l as i16]
        }
    }
}
