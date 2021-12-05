//! Utility newtypes for working with angles in both
//! degrees and radians.
//!
//! Functions can use `Into<Rad<f32>>` or `Into<Rad<f64>>`
//! to support both `Deg` and float literals.
//!
//! # Example
//!
//! ```
//! use angle::{Deg, Rad};
//! use std::f32::consts::PI;
//!
//! fn add_pi<A: Into<Rad<f32>>>(angle: A) -> f32 {
//!     angle.into().value() + PI
//! }
//!
//! assert_eq!(PI * 2.0, add_pi(PI));
//! assert_eq!(PI * 2.0, add_pi(Deg(180.)));
//! ```
use num_traits::{
    cast::FromPrimitive,
    float::{Float, FloatConst},
};
use std::fmt;

// NOTE: repr(transparent) is for C ffi, required so
//       Rust will use the correct C calling conventions.
//       The struct is treated as if it were the same type
//       as its internal value.
//       Otherwise Rust and C could store the value
//       in different registers on some platforms.

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Deg<N: Float>(pub N);

impl<N> Deg<N>
where
    N: Float + FromPrimitive + FloatConst,
{
    #[inline]
    pub fn value(&self) -> N {
        self.0
    }

    #[inline]
    pub fn to_radians(&self) -> N {
        let d: N = N::from_f64(180.).unwrap();
        let pi = N::PI();
        self.0 * (pi / d)
    }

    #[inline]
    pub fn approx_eq<T: Into<Self>>(&self, rhs: T) -> bool {
        (self.0 - rhs.into().0).abs() < Float::epsilon()
    }
}

impl<N> Into<Rad<N>> for Deg<N>
where
    N: Float + FromPrimitive + FloatConst,
{
    fn into(self) -> Rad<N> {
        Rad(self.to_radians())
    }
}

/// Convert float to degrees.
impl<N: Float> From<N> for Deg<N> {
    #[inline]
    fn from(value: N) -> Deg<N> {
        Deg(value)
    }
}

impl<N> fmt::Display for Deg<N>
where
    N: Float + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Rad<N: Float>(pub N);

impl<N> Rad<N>
where
    N: Float + FromPrimitive + FloatConst,
{
    #[inline]
    pub fn value(&self) -> N {
        self.0
    }

    #[inline]
    pub fn to_degrees(&self) -> N {
        let d: N = N::from_f64(180.).unwrap();
        let pi = N::PI();
        self.0 * (d / pi)
    }

    #[inline]
    pub fn approx_eq<T: Into<Self>>(&self, rhs: T) -> bool {
        (self.0 - rhs.into().0).abs() < Float::epsilon()
    }
}

impl<N> Into<Deg<N>> for Rad<N>
where
    N: Float + FromPrimitive + FloatConst,
{
    #[inline]
    fn into(self) -> Deg<N> {
        Deg(self.to_degrees())
    }
}

/// Convert float to radians.
impl<N: Float> From<N> for Rad<N> {
    #[inline]
    fn from(value: N) -> Rad<N> {
        Rad(value)
    }
}

impl<N> fmt::Display for Rad<N>
where
    N: Float + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

/// Approximate equality comparison for floating point numbers.
#[macro_export]
macro_rules! inexact_eq {
    ($lhs:expr, $rhs:expr) => {
        (f64::from($lhs) - f64::from($rhs)).abs() < std::f64::EPSILON
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::inexact_eq;

    #[test]
    fn test_degrees() {
        let deg_45 = Deg(45.);
        let rad_45 = ::std::f32::consts::PI / 4.;
        assert!(inexact_eq!(rad_45, deg_45.to_radians()));
        assert!(Rad(rad_45).approx_eq(deg_45));
    }

    #[test]
    fn test_radians() {
        let pi = ::std::f32::consts::PI;
        let deg_45 = 45.;
        let rad_45 = Rad(pi / 4.);
        assert!(inexact_eq!(deg_45, rad_45.to_degrees()));
        assert!(Deg(deg_45).approx_eq(rad_45));
    }

    /// Using a float directly as degrees or rads.
    #[test]
    fn test_info() {
        fn sum_f32<T: Into<Deg<f32>>>(lhs: T, rhs: T) -> Deg<f32> {
            Deg(lhs.into().value() + rhs.into().value())
        }
        assert_eq!(sum_f32(Deg(45.), Deg(75.)), Deg(120.));
        assert_eq!(sum_f32(45., 75.), Deg(120.));
    }

    #[test]
    fn test_non_generic() {
        type Vector = [f64; 2];

        /// Rotate vector counterclockwise by the given angle
        fn rotate(a: Vector, angle: Deg<f64>) -> Vector {
            let r = angle.to_radians();
            let x = a[0] * f64::cos(r) - a[1] * f64::sin(r);
            let y = a[0] * f64::sin(r) + a[1] * f64::cos(r);
            [x, y]
        }

        let actual = rotate([1., 0.], Deg(90.));
        let expected = [0., 1.];
        assert!(inexact_eq!(actual[0], expected[0]));
        assert!(inexact_eq!(actual[1], expected[1]));
    }

    #[test]
    fn test_generic() {
        type Vector = [f64; 2];

        /// Rotate vector counterclockwise by the given angle
        fn rotate<T>(a: Vector, angle: T) -> Vector
        where
            T: Into<Deg<f64>>,
        {
            let r = angle.into().to_radians();
            let x = a[0] * f64::cos(r) - a[1] * f64::sin(r);
            let y = a[0] * f64::sin(r) + a[1] * f64::cos(r);
            [x, y]
        }

        let expected = [0., 1.];

        {
            let actual = rotate([1., 0.], 90.);
            assert!(inexact_eq!(actual[0], expected[0]));
            assert!(inexact_eq!(actual[1], expected[1]));
        }

        {
            let actual = rotate([1., 0.], Deg(90.));
            assert!(inexact_eq!(actual[0], expected[0]));
            assert!(inexact_eq!(actual[1], expected[1]));
        }

        {
            let actual = rotate([1., 0.], Rad(std::f64::consts::PI / 2.));
            assert!(inexact_eq!(actual[0], expected[0]));
            assert!(inexact_eq!(actual[1], expected[1]));
        }
    }
}
