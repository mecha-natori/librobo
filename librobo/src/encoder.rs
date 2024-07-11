//! ロータリーエンコーダー補助モジュール。

mod ffi;

/// ロータリーエンコーダー。
pub struct Encoder {
    cpr: u16,
    count: i64
}

impl Encoder {
    /// 分解能からインスタンスを作成する。
    pub fn new(ppr: u16) -> Self {
        Self {
            cpr: ppr * 4,
            count: 0
        }
    }

    /// カウンタを更新する。
    pub fn update(&mut self, delta: i64) {
        self.count += delta;
    }

    /// 生のカウンタを取得する。
    pub fn get_count(&self) -> i64 {
        self.count
    }

    /// 角度を60分法で取得する。
    pub fn get_degree(&self) -> f64 {
        (self.count / self.cpr as i64) as f64 * 360f64
    }

    /// 角度を弧度法で取得する。
    pub fn get_radian(&self) -> f64 {
        (self.count / self.cpr as i64) as f64 * 2f64 * std::f64::consts::PI
    }

    /// 回転回数を取得する。
    pub fn get_revolution(&self) -> i64 {
        self.count / self.cpr as i64
    }
}
