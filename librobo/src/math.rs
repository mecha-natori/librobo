//! 数学補助モジュール

use num::Complex;
use num::Float;

/// 直行座標系
pub trait Cartesian<T> {
    /// X座標を返す。
    ///
    /// # Examples
    /// ```
    /// use num::Complex;
    /// use robo::math::Cartesian;
    ///
    /// let c: Complex<i32> = Complex::new(10, 5);
    /// assert_eq!(c.x(), 10);
    /// ```
    fn x(&self) -> T;

    /// Y座標を返す。
    ///
    /// # Examples
    /// ```
    /// use num::Complex;
    /// use robo::math::Cartesian;
    ///
    /// let c: Complex<i32> = Complex::new(10, 5);
    /// assert_eq!(c.y(), 5);
    /// ```
    fn y(&self) -> T;
}

impl<T> Cartesian<T> for Complex<T>
where
    T: Copy
{
    fn x(&self) -> T {
        self.re
    }

    fn y(&self) -> T {
        self.im
    }
}

/// 浮動小数点数の近似等価演算
pub trait FloatApproximately<T>
where
    T: Float
{
    /// 2つの浮動小数点数がおおよそ等しいかを返す。
    fn approximately(&self, other: &Self) -> bool;
}

macro_rules! impl_float_approximately {
    ($($t:ty),*) => {
        $(
            impl FloatApproximately<$t> for $t {
                fn approximately(&self, other: &Self) -> bool {
                    (self - other).abs() < Self::EPSILON
                }
            }
        )*
    };
}

impl_float_approximately!(f32, f64);

/// 線形補間の逆関数
pub trait InverseLerp<T> {
    /// 線形補間の逆演算を行う。
    fn inverse_lerp(from: T, to: T, value: T) -> T;
}

macro_rules! impl_inverse_lerp {
    ($($t:ty),*) => {
        $(
            impl InverseLerp<$t> for $t {
                fn inverse_lerp(from: $t, to: $t, value: $t) -> $t {
                    (value - from) / (to - from)
                }
            }
        )*
    };
}

impl_inverse_lerp!(f32, f64, i128, i16, i32, i64, i8, u128, u16, u32, u64, u8);
