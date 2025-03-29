//! コントローラー入力補助モジュール

use crate::math::Cartesian;
use crate::math::InverseLerp;
use crate::util::debug_log;
use crate::util::trace_log;
use num::Bounded;
use num::Float;
use num::Integer;
use num::Num;
use num::NumCast;
use num::Signed;
use num::ToPrimitive;
use num::Unsigned;

/// 左右スティック
///
/// 各スティックのX軸は右が正であり、Y軸は上が正である。
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
///
/// 各スティックのX軸は右が正であり、Y軸は上が正である。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct NormalizedSticks {
    /// 左スティック
    pub l: [f32; 2],
    /// 右スティック
    pub r: [f32; 2]
}

/// 左右スティックの入力を正規化する。
pub fn normalize_sticks(sticks: Sticks) -> NormalizedSticks {
    debug_log!(target: "librobo/controller", "normalize sticks: {:?}", sticks);
    let dead_zone = sticks.dead_zone as f32 / 100f32;
    let lx = sticks.l[0] as f32 / i16::MAX as f32;
    let ly = sticks.l[1] as f32 / i16::MAX as f32;
    trace_log!(target: "librobo/controller", "LX: {}, LY: {}", lx, ly);
    let lr = lx.hypot(ly);
    let lx = if lr <= dead_zone {
        0f32
    } else if 1f32 < lr {
        lx / lr
    } else {
        lx * f32::inverse_lerp(dead_zone, 1f32, lr) / lr
    };
    let ly = if lr <= dead_zone {
        0f32
    } else if 1f32 < lr {
        ly / lr
    } else {
        ly * f32::inverse_lerp(dead_zone, 1f32, lr) / lr
    };
    trace_log!(target: "librobo/controller", "LX: {}, LY: {}", lx, ly);
    let rx = sticks.r[0] as f32 / i16::MAX as f32;
    let ry = sticks.r[1] as f32 / i16::MAX as f32;
    trace_log!(target: "librobo/controller", "RX: {}, RY: {}", rx, ry);
    let rr = rx.hypot(ry);
    let rx = if rr <= dead_zone {
        0f32
    } else if 1f32 < rr {
        rx / rr
    } else {
        rx * f32::inverse_lerp(dead_zone, 1f32, rr) / rr
    };
    let ry = if rr <= dead_zone {
        0f32
    } else if 1f32 < rr {
        ry / rr
    } else {
        ry * f32::inverse_lerp(dead_zone, 1f32, rr) / rr
    };
    trace_log!(target: "librobo/controller", "RX: {}, RY: {}", rx, ry);
    let normalized = NormalizedSticks {
        l: [lx, ly],
        r: [rx, ry]
    };
    debug_log!(target: "librobo/controller", "normalized sticks: {:?}", normalized);
    normalized
}

/// 左右スティック入力がそれぞれデッドゾーンに「入っているか」を判定する。
///
/// デッドゾーンは中心からの距離をパーセントで指定する。
/// 戻り値は\[Left X, Left Y, Right X, Right Y]。
pub fn is_sticks_in_dead_zone(sticks: Sticks) -> [bool; 4] {
    debug_log!(target: "librobo/controller", "check sticks is in dead zone: {:?}", sticks);
    let lx = sticks.l[0].abs() as f32 / i16::MAX as f32 <= sticks.dead_zone as f32 / 100f32;
    let ly = sticks.l[1].abs() as f32 / i16::MAX as f32 <= sticks.dead_zone as f32 / 100f32;
    let rx = sticks.r[0].abs() as f32 / i16::MAX as f32 <= sticks.dead_zone as f32 / 100f32;
    let ry = sticks.r[1].abs() as f32 / i16::MAX as f32 <= sticks.dead_zone as f32 / 100f32;
    let result = [lx, ly, rx, ry];
    debug_log!(target: "librobo/controller", "check result: {:?}", result);
    result
}

/// 正規化された左右スティック入力がそれぞれデッドゾーンに「入っているか」を判定する。
///
/// デッドゾーンは中心からの距離をパーセントで指定する。
/// 戻り値は\[Left X, Left Y, Right X, Right Y]。
#[deprecated(
    note = "正規化時にチェックするよう変更したため常にfalseを返す。",
    since = "0.4.0"
)]
pub fn is_normalized_sticks_in_dead_zone(sticks: NormalizedSticks) -> [bool; 4] {
    debug_log!(target: "librobo/controller", "check normalized sticks is in dead zone: {:?}", sticks);
    [false, false, false, false]
}

/// 左右スティック入力のデッドゾーンを処理する。
///
/// 各スティック入力を各軸ごとに読み取り、デッドゾーン内であれば0に置き換える。
pub fn process_sticks_dead_zone(sticks: Sticks) -> Sticks {
    debug_log!(target: "librobo/controller", "process each stick's dead zone: {:?}", sticks);
    let is_in_dead_zone = is_sticks_in_dead_zone(sticks);
    let processed = Sticks {
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
    };
    debug_log!(target: "librobo/controller", "processed sticks: {:?}", processed);
    processed
}

/// 正規化された左右スティック入力のデッドゾーンを処理する。
///
/// 各スティック入力を各軸ごとに読み取り、デッドゾーン内であれば0に置き換える。
#[deprecated(
    note = "正規化時にチェックするよう変更したため常に入力を返す。",
    since = "0.4.0"
)]
pub fn process_normalized_sticks_dead_zone(sticks: NormalizedSticks) -> NormalizedSticks {
    sticks
}
