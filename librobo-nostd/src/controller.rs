//! コントローラー入力補助モジュール。

use num::{Bounded, Complex, Float, Integer, Num, NumCast, Signed, ToPrimitive, Unsigned};

use crate::math::Cartesian;

/// 左右スティック。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sticks<T>
where
    T: Num + Signed
{
    /// 左スティック。
    pub l: Complex<T>,
    /// 右スティック。
    pub r: Complex<T>,
    /// 0-1か。
    pub is_normalized: bool
}

impl<T> Sticks<T>
where
    T: Bounded + Copy + Num + PartialOrd + Signed + ToPrimitive
{
    /// 各スティックの生の値からインスタンスを作成する。
    ///
    /// 値の型の最大値を入力の最大値として計算します。
    pub fn new(lx: T, ly: T, rx: T, ry: T, is_normalized: Option<bool>) -> Self {
        Self {
            l: Complex::new(lx, ly),
            r: Complex::new(rx, ry),
            is_normalized: is_normalized.unwrap_or(false)
        }
    }

    /// 各スティックの直交座標からインスタンスを作成する。
    pub fn from_complex(l: Complex<T>, r: Complex<T>, is_normalized: Option<bool>) -> Self {
        Self {
            l,
            r,
            is_normalized: is_normalized.unwrap_or(false)
        }
    }

    /// 各スティックの値を0-1へ変換する。
    pub fn normalize<U>(&self) -> Sticks<U>
    where
        U: Bounded + Float + NumCast + Signed
    {
        if self.is_normalized {
            return Sticks::new(
                U::from(self.l.x()).unwrap_or(U::zero()),
                U::from(self.l.y()).unwrap_or(U::zero()),
                U::from(self.r.x()).unwrap_or(U::zero()),
                U::from(self.r.y()).unwrap_or(U::zero()),
                Some(true)
            );
        }
        let lx = if self.l.x() < T::zero() {
            self.l.x() + T::one()
        } else {
            self.l.x()
        };
        let ly = if self.l.y() < T::zero() {
            self.l.y() + T::one()
        } else {
            self.l.y()
        };
        let rx = if self.r.x() < T::zero() {
            self.r.x() + T::one()
        } else {
            self.r.x()
        };
        let ry = if self.r.y() < T::zero() {
            self.r.y() + T::one()
        } else {
            self.r.y()
        };
        Sticks::new(
            self._normalize(lx),
            self._normalize(ly),
            self._normalize(rx),
            self._normalize(ry),
            Some(true)
        )
    }

    fn _normalize<U>(&self, value: T) -> U
    where
        U: Float + NumCast
    {
        U::from(value).unwrap_or(U::zero()) / U::from(T::max_value()).unwrap_or(U::one())
    }
}

/// コントローラーユーティリティー。
#[derive(Debug, Default)]
pub struct Controller<T>
where
    T: Copy + ToPrimitive + Unsigned
{
    /// スティックのデッドゾーン。単位はパーセント。
    dead_zone: T
}

impl<T> Controller<T>
where
    T: Copy + ToPrimitive + Unsigned
{
    /// インスタンスを作成する。
    pub fn new(dead_zone: T) -> Self {
        Self { dead_zone }
    }

    /// スティック入力を処理する。
    ///
    /// 生のスティック入力をデッドゾーン判定にかけて0-1の値に変換します。
    pub fn process_sticks<U, V>(&self, sticks: Sticks<U>) -> Sticks<V>
    where
        U: Bounded + Copy + Integer + Signed + ToPrimitive,
        V: Bounded + Float + Signed
    {
        let sticks = sticks.normalize::<V>();
        let lx = if self.is_in_deadzone::<U, V>(sticks.l.x()) {
            V::zero()
        } else {
            sticks.l.x()
        };
        let ly = if self.is_in_deadzone::<U, V>(sticks.l.y()) {
            V::zero()
        } else {
            sticks.l.y()
        };
        let rx = if self.is_in_deadzone::<U, V>(sticks.r.x()) {
            V::zero()
        } else {
            sticks.r.x()
        };
        let ry = if self.is_in_deadzone::<U, V>(sticks.r.y()) {
            V::zero()
        } else {
            sticks.r.y()
        };
        Sticks::new(lx, ly, rx, ry, Some(true))
    }

    fn is_in_deadzone<U, V>(&self, value: V) -> bool
    where
        U: Bounded + ToPrimitive,
        V: Float + NumCast
    {
        V::from(value.abs()).unwrap_or(V::zero()) <=
            V::from(self.dead_zone).unwrap_or(V::zero()) / V::from(100).unwrap_or(V::one()) *
                V::from(U::max_value()).unwrap_or(V::zero())
    }
}
