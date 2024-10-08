//! サーボ補助モジュール

#[cfg(feature = "bind-c")]
mod ffi;

/// サーボの情報
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ServoDefinition {
    /// 最小のパルス幅 \[ms]
    pub min_ms: u16,
    /// 最大のパルス幅 \[ms]
    pub max_ms: u16,
    /// 最小の角度 \[deg]
    pub min_deg: i16,
    /// 最大の角度 \[deg]
    pub max_deg: i16
}

/// 角度 \[deg]からサーボのPWMデューティを計算する。
///
/// # Arguments
/// deg      - 角度 \[deg]
/// max_duty - 100%時のデューティの値
/// servo    - サーボの仕様データ
pub fn calc_servo_duty(deg: i16, max_duty: u16, servo: ServoDefinition) -> u16 {
    ((deg - servo.min_deg) / (servo.max_deg - servo.min_deg)) as u16 * (servo.max_ms - servo.min_ms) +
        servo.min_ms
}
