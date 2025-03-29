//! ロータリーエンコーダー補助モジュール

use crate::util::debug_log;

/// ロータリーエンコーダー
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Encoder {
    /// 分解能
    pub ppr: u16,
    /// カウンタ
    pub count: i64
}

/// カウンタを更新する。
pub fn update(encoder: Encoder, delta: i64) -> Encoder {
    debug_log!(target: "librobo/encoder", "update counter with: {:+}", delta);
    let count = encoder.count + delta;
    debug_log!(target: "librobo/encoder", "updated counter: {:+}", count);
    Encoder {
        count,
        ..encoder
    }
}

/// 角度を60分法で取得する。
pub fn get_degree(encoder: Encoder) -> f64 {
    let deg = encoder.count as f64 / encoder.ppr as f64 / 4f64 * 360f64;
    debug_log!(target: "librobo/encoder", "get degree: {}", deg);
    deg
}

/// 角度を弧度法で取得する。
pub fn get_radian(encoder: Encoder) -> f64 {
    let rad = encoder.count as f64 / encoder.ppr as f64 / 4f64 * 2f64 * core::f64::consts::PI;
    debug_log!(target: "librobo/encoder", "get degree: {}", rad);
    rad
}

/// 回転回数を取得する。
pub fn get_revolution(encoder: Encoder) -> i64 {
    let revl = encoder.count / encoder.ppr as i64 / 4;
    debug_log!(target: "librobo/encoder", "get revolution: {}", revl);
    revl
}
