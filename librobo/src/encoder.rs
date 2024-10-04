//! ロータリーエンコーダー補助モジュール

#[cfg(feature = "bind-c")]
mod ffi;

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
    Encoder {
        count: encoder.count + delta,
        ..encoder
    }
}

/// 角度を60分法で取得する。
pub fn get_degree(encoder: Encoder) -> f64 {
    encoder.count as f64 / encoder.ppr as f64 / 4f64 * 360f64
}

/// 角度を弧度法で取得する。
pub fn get_radian(encoder: Encoder) -> f64 {
    encoder.count as f64 / encoder.ppr as f64 / 4f64 * 2f64 * core::f64::consts::PI
}

/// 回転回数を取得する。
pub fn get_revolution(encoder: Encoder) -> i64 {
    encoder.count / encoder.ppr as i64 / 4
}
