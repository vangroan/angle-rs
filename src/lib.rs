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

impl From<Deg<f32>> for f64 {
    #[inline]
    fn from(deg: Deg<f32>) -> f64 {
        f64::from(deg.0)
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

impl Into<Rad<f32>> for f32 {
    #[inline]
    fn into(self) -> Rad<f32> {
        Rad(self)
    }
}

impl From<Rad<f32>> for f64 {
    #[inline]
    fn from(rad: Rad<f32>) -> f64 {
        f64::from(rad.0)
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
}
