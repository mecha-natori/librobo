//! 数学補助モジュール。

use num::Complex;

/// 直行座標系
pub trait Cartesian<T> {
    /// X座標を返す。
    ///
    /// # Examples
    /// ```
    /// use librobo::math::Cartesian;
    /// use num::Complex;
    ///
    /// let c: Complex<i32> = Complex::new(10, 5);
    /// assert_eq!(c.x(), 10);
    /// ```
    fn x(&self) -> T;

    /// Y座標を返す。
    ///
    /// # Examples
    /// ```
    /// use librobo::math::Cartesian;
    /// use num::Complex;
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
