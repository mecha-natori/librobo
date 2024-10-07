//! esp32_ds4_driverとの連携モジュール。

use esp32_ds4_driver::Sticks;
use robo::controller::Sticks as RoboSticks;

/// [`Sticks`][RoboSticks]への変換を提供。
pub trait ToRoboSticks {
    /// デッドゾーン情報と自身から[`Sticks`][RoboSticks]へ変換する。
    fn to_robo_sticks(&self, dead_zone: u8) -> RoboSticks;
}

impl ToRoboSticks for Sticks {
    fn to_robo_sticks(&self, dead_zone: u8) -> RoboSticks {
        RoboSticks {
            l: [self.lx as i16 * 0xffi16, self.ly as i16 * 0xffi16],
            r: [self.rx as i16 * 0xffi16, self.ry as i16 * 0xffi16],
            dead_zone
        }
    }
}
