//! クローラーモジュール

use super::ISteering;
#[cfg(feature = "controller")]
use super::ISteeringFromSticks;
use super::PIDData;
use super::Steering;
use super::process_pid_data;
use crate::util::debug_log;
use crate::util::trace_log;
use num::Complex;

const N: usize = 2;

/// クローラー
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Crawler;

impl ISteering<N> for Crawler {
    fn calc_speed(
        steering: Steering,
        pid_data: Option<&mut [PIDData; N]>,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> [i16; N] {
        debug_log!(target: "librobo/steering/crawler", "calculate speed: L: {}, R: {}", l, r);
        let r = steering.max_speed as f32 * -r.im;
        let l = steering.max_speed as f32 * l.im;
        trace_log!(target: "librobo/steering/crawler", "L: {}, R: {}", l, r);
        let result = if let Some(mut pid_data) = pid_data {
            debug_log!(target: "librobo/steering/crawler", "found PID data: {:?}", pid_data);
            let r = process_pid_data(&mut pid_data[0], r);
            let l = process_pid_data(&mut pid_data[1], l);
            trace_log!(target: "librobo/steering/crawler", "L: {}, R: {}", l, r);
            [r as i16, l as i16]
        } else {
            [r as i16, l as i16]
        };
        debug_log!(target: "librobo/steering/crawler", "calculated speed: {:?}", result);
        result
    }
}
