//! コントローラー入力補助モジュール

use crate::math::Cartesian;
use num::Bounded;
use num::Float;
use num::Integer;
use num::Num;
use num::NumCast;
use num::Signed;
use num::ToPrimitive;
use num::Unsigned;

#[cfg(feature = "bind-c")]
mod ffi;

/// 左右スティック
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Sticks {
    /// 左スティック
    pub l: [i16; 2],
    /// 右スティック
    pub r: [i16; 2],
    /// デッドゾーン \[%]
    pub dead_zone: u8
}

/// -1 - 1に正規化された左右スティック
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct NormalizedSticks {
    /// 左スティック
    pub l: [f32; 2],
    /// 右スティック
    pub r: [f32; 2],
    /// デッドゾーン \[%]
    pub dead_zone: u8
}

/// 左右スティックの入力を正規化する。
pub fn normalize_sticks(sticks: Sticks) -> NormalizedSticks {
    let lx = sticks.l[0] as f32 / i16::MAX as f32;
    let ly = sticks.l[1] as f32 / i16::MAX as f32;
    let lr = f32::hypot(lx, ly);
    let ltheta = f32::atan2(ly, lx);
    let lr = (lr / (ltheta.sin() + ltheta.cos())).abs();
    let lx = lr * ltheta.cos();
    let ly = lr * ltheta.sin();
    let rx = sticks.r[0] as f32 / i16::MAX as f32;
    let ry = sticks.r[1] as f32 / i16::MAX as f32;
    let rr = f32::hypot(rx, ry);
    let rtheta = f32::atan2(ry, rx);
    let rr = (rr / (rtheta.sin() + rtheta.cos())).abs();
    let rx = rr * rtheta.cos();
    let ry = rr * rtheta.sin();
    NormalizedSticks {
        l: [lx, ly],
        r: [rx, ry],
        dead_zone: sticks.dead_zone
    }
}

/// 左右スティック入力がそれぞれデッドゾーンに「入っているか」を判定する。
///
/// デッドゾーンは中心からの距離をパーセントで指定する。
/// 戻り値は\[Left X, Left Y, Right X, Right Y]。
pub fn is_sticks_in_dead_zone(sticks: Sticks) -> [bool; 4] {
    is_normalized_sticks_in_dead_zone(normalize_sticks(sticks))
}

/// 正規化された左右スティック入力がそれぞれデッドゾーンに「入っているか」を判定する。
///
/// デッドゾーンは中心からの距離をパーセントで指定する。
/// 戻り値は\[Left X, Left Y, Right X, Right Y]。
pub fn is_normalized_sticks_in_dead_zone(sticks: NormalizedSticks) -> [bool; 4] {
    let lx = sticks.l[0].abs() <= sticks.dead_zone as f32 / 100f32;
    let ly = sticks.l[1].abs() <= sticks.dead_zone as f32 / 100f32;
    let rx = sticks.r[0].abs() <= sticks.dead_zone as f32 / 100f32;
    let ry = sticks.r[1].abs() <= sticks.dead_zone as f32 / 100f32;
    [lx, ly, rx, ry]
}

/// 左右スティック入力のデッドゾーンを処理する。
///
/// 各スティック入力を各軸ごとに読み取り、デッドゾーン内であれば0に置き換える。
pub fn process_sticks_dead_zone(sticks: Sticks) -> Sticks {
    let is_in_dead_zone = is_sticks_in_dead_zone(sticks);
    Sticks {
        l: [
            if is_in_dead_zone[0] {
                0i16
            } else {
                sticks.l[0]
            },
            if is_in_dead_zone[1] {
                0i16
            } else {
                sticks.l[1]
            }
        ],
        r: [
            if is_in_dead_zone[2] {
                0i16
            } else {
                sticks.r[0]
            },
            if is_in_dead_zone[3] {
                0i16
            } else {
                sticks.r[1]
            }
        ],
        ..sticks
    }
}

/// 正規化された左右スティック入力のデッドゾーンを処理する。
///
/// 各スティック入力を各軸ごとに読み取り、デッドゾーン内であれば0に置き換える。
pub fn process_normalized_sticks_dead_zone(sticks: NormalizedSticks) -> NormalizedSticks {
    let is_in_dead_zone = is_normalized_sticks_in_dead_zone(sticks);
    NormalizedSticks {
        l: [
            if is_in_dead_zone[0] {
                0f32
            } else {
                sticks.l[0]
            },
            if is_in_dead_zone[1] {
                0f32
            } else {
                sticks.l[1]
            }
        ],
        r: [
            if is_in_dead_zone[2] {
                0f32
            } else {
                sticks.r[0]
            },
            if is_in_dead_zone[3] {
                0f32
            } else {
                sticks.r[1]
            }
        ],
        ..sticks
    }
}
