//! 四輪オムニホイールモジュール

use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use super::process_pid_data;
use crate::controller::NormalizedSticks;
use crate::util::debug_log;
use crate::util::trace_log;
use num::Complex;
#[cfg(not(feature = "std"))]
use num::Float;
use num::traits::FloatConst;

#[cfg(feature = "bind-c")]
mod ffi;

const N: usize = 4;

/// 四輪オムニホイール
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QuadOmni;

impl ISteering<N> for QuadOmni {
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut [PIDData; N]>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> [i16; N] {
        debug_log!(target: "librobo/steering/quad_omni", "calculate speed: L: {}, R: {}", l, r);
        let (l_r, l_theta) = l.to_polar();
        let (r_r, r_theta) = r.to_polar();
        trace_log!(target: "librobo/steering/quad_omni", "Lr: {}, Ltheta: {}, Rr: {}, Rtheta: {}", l_r, l_theta, r_r, r_theta);
        let fr = steering.max_speed as f32 * r_r * (f32::FRAC_PI_4() + r_theta).cos();
        let fl = steering.max_speed as f32 * l_r * (f32::FRAC_PI_4() - l_theta).cos();
        let rl = steering.max_speed as f32 * l_r * (3f32 * f32::FRAC_PI_4() - l_theta).cos();
        let rr = steering.max_speed as f32 * r_r * (3f32 * f32::FRAC_PI_4() + r_theta).cos();
        trace_log!(target: "librobo/steering/quad_omni", "FR: {}, FL: {}, RL: {}, RR: {}", fr, fl, rl, rr);
        let result = if let Some(mut pid_data) = pid_data {
            debug_log!(target: "librobo/steering/quad_omni", "found PID data: {:?}", pid_data);
            let fr = process_pid_data(&mut pid_data[0], fr);
            let fl = process_pid_data(&mut pid_data[1], fl);
            let rl = process_pid_data(&mut pid_data[2], rl);
            let rr = process_pid_data(&mut pid_data[3], rr);
            [fr as i16, fl as i16, rl as i16, rr as i16]
        } else {
            [fr as i16, fl as i16, rl as i16, rr as i16]
        };
        debug_log!(target: "librobo/steering/quad_omni", "calculated speed: {:?}", result);
        result
    }
}
