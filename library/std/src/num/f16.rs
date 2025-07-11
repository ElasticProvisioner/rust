//! Constants for the `f16` half-precision floating point type.
//!
//! *[See also the `f16` primitive type](primitive@f16).*
//!
//! Mathematically significant numbers are provided in the `consts` sub-module.

#![unstable(feature = "f16", issue = "116909")]
#![doc(test(attr(feature(cfg_target_has_reliable_f16_f128), expect(internal_features))))]

#[unstable(feature = "f16", issue = "116909")]
pub use core::f16::consts;

#[cfg(not(test))]
use crate::intrinsics;
#[cfg(not(test))]
use crate::sys::cmath;

#[cfg(not(test))]
impl f16 {
    /// Raises a number to a floating point power.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 2.0_f16;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    /// assert!(abs_difference <= f16::EPSILON);
    ///
    /// assert_eq!(f16::powf(1.0, f16::NAN), 1.0);
    /// assert_eq!(f16::powf(f16::NAN, 0.0), 1.0);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn powf(self, n: f16) -> f16 {
        unsafe { intrinsics::powf16(self, n) }
    }

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let one = 1.0f16;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp(self) -> f16 {
        unsafe { intrinsics::expf16(self) }
    }

    /// Returns `2^(self)`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let f = 2.0f16;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp2(self) -> f16 {
        unsafe { intrinsics::exp2f16(self) }
    }

    /// Returns the natural logarithm of the number.
    ///
    /// This returns NaN when the number is negative, and negative infinity when number is zero.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let one = 1.0f16;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    ///
    /// Non-positive values:
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// assert_eq!(0_f16.ln(), f16::NEG_INFINITY);
    /// assert!((-42_f16).ln().is_nan());
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln(self) -> f16 {
        unsafe { intrinsics::logf16(self) }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// This returns NaN when the number is negative, and negative infinity when number is zero.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and
    /// `self.log10()` can produce more accurate results for base 10.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let five = 5.0f16;
    ///
    /// // log5(5) - 1 == 0
    /// let abs_difference = (five.log(5.0) - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    ///
    /// Non-positive values:
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// assert_eq!(0_f16.log(10.0), f16::NEG_INFINITY);
    /// assert!((-42_f16).log(10.0).is_nan());
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log(self, base: f16) -> f16 {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// This returns NaN when the number is negative, and negative infinity when number is zero.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let two = 2.0f16;
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference = (two.log2() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    ///
    /// Non-positive values:
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// assert_eq!(0_f16.log2(), f16::NEG_INFINITY);
    /// assert!((-42_f16).log2().is_nan());
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(self) -> f16 {
        unsafe { intrinsics::log2f16(self) }
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// This returns NaN when the number is negative, and negative infinity when number is zero.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let ten = 10.0f16;
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference = (ten.log10() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    ///
    /// Non-positive values:
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// assert_eq!(0_f16.log10(), f16::NEG_INFINITY);
    /// assert!((-42_f16).log10().is_nan());
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log10(self) -> f16 {
        unsafe { intrinsics::log10f16(self) }
    }

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `hypotf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 2.0f16;
    /// let y = 3.0f16;
    ///
    /// // sqrt(x^2 + y^2)
    /// let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn hypot(self, other: f16) -> f16 {
        cmath::hypotf(self as f32, other as f32) as f16
    }

    /// Computes the sine of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = std::f16::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sin(self) -> f16 {
        unsafe { intrinsics::sinf16(self) }
    }

    /// Computes the cosine of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 2.0 * std::f16::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn cos(self) -> f16 {
        unsafe { intrinsics::cosf16(self) }
    }

    /// Computes the tangent of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `tanf` from libc on Unix and
    /// Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = std::f16::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn tan(self) -> f16 {
        cmath::tanf(self as f32) as f16
    }

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `asinf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let f = std::f16::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f16::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arcsin")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn asin(self) -> f16 {
        cmath::asinf(self as f32) as f16
    }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `acosf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let f = std::f16::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f16::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arccos")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn acos(self) -> f16 {
        cmath::acosf(self as f32) as f16
    }

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `atanf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let f = 1.0f16;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arctan")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atan(self) -> f16 {
        cmath::atanf(self as f32) as f16
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `atan2f` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0f16;
    /// let y1 = -3.0f16;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0f16;
    /// let y2 = 3.0f16;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-std::f16::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f16::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= f16::EPSILON);
    /// assert!(abs_difference_2 <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atan2(self, other: f16) -> f16 {
        cmath::atan2f(self as f32, other as f32) as f16
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `(f16::sin(x),
    /// f16::cos(x))`. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = std::f16::consts::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 <= f16::EPSILON);
    /// assert!(abs_difference_1 <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "sincos")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    pub fn sin_cos(self) -> (f16, f16) {
        (self.sin(), self.cos())
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `expm1f` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 1e-4_f16;
    ///
    /// // for very small x, e^x is approximately 1 + x + x^2 / 2
    /// let approx = x + x * x / 2.0;
    /// let abs_difference = (x.exp_m1() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-4);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp_m1(self) -> f16 {
        cmath::expm1f(self as f32) as f16
    }

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// This returns NaN when `n < -1.0`, and negative infinity when `n == -1.0`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `log1pf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 1e-4_f16;
    ///
    /// // for very small x, ln(1 + x) is approximately x - x^2 / 2
    /// let approx = x - x * x / 2.0;
    /// let abs_difference = (x.ln_1p() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-4);
    /// # }
    /// ```
    ///
    /// Out-of-range values:
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// assert_eq!((-1.0_f16).ln_1p(), f16::NEG_INFINITY);
    /// assert!((-2.0_f16).ln_1p().is_nan());
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "log1p")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln_1p(self) -> f16 {
        cmath::log1pf(self as f32) as f16
    }

    /// Hyperbolic sine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `sinhf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let e = std::f16::consts::E;
    /// let x = 1.0f16;
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sinh(self) -> f16 {
        cmath::sinhf(self as f32) as f16
    }

    /// Hyperbolic cosine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `coshf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let e = std::f16::consts::E;
    /// let x = 1.0f16;
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn cosh(self) -> f16 {
        cmath::coshf(self as f32) as f16
    }

    /// Hyperbolic tangent function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `tanhf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let e = std::f16::consts::E;
    /// let x = 1.0f16;
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn tanh(self) -> f16 {
        cmath::tanhf(self as f32) as f16
    }

    /// Inverse hyperbolic sine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 1.0f16;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arcsinh")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn asinh(self) -> f16 {
        let ax = self.abs();
        let ix = 1.0 / ax;
        (ax + (ax / (Self::hypot(1.0, ix) + ix))).ln_1p().copysign(self)
    }

    /// Inverse hyperbolic cosine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 1.0f16;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arccosh")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn acosh(self) -> f16 {
        if self < 1.0 {
            Self::NAN
        } else {
            (self + ((self - 1.0).sqrt() * (self + 1.0).sqrt())).ln()
        }
    }

    /// Inverse hyperbolic tangent function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let e = std::f16::consts::E;
    /// let f = e.tanh().atanh();
    ///
    /// let abs_difference = (f - e).abs();
    ///
    /// assert!(abs_difference <= 0.01);
    /// # }
    /// ```
    #[inline]
    #[doc(alias = "arctanh")]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atanh(self) -> f16 {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }

    /// Gamma function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `tgammaf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// #![feature(float_gamma)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 5.0f16;
    ///
    /// let abs_difference = (x.gamma() - 24.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    // #[unstable(feature = "float_gamma", issue = "99842")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn gamma(self) -> f16 {
        cmath::tgammaf(self as f32) as f16
    }

    /// Natural logarithm of the absolute value of the gamma function
    ///
    /// The integer part of the tuple indicates the sign of the gamma function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `lgamma_r` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// #![feature(float_gamma)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    ///
    /// let x = 2.0f16;
    ///
    /// let abs_difference = (x.ln_gamma().0 - 0.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[inline]
    #[rustc_allow_incoherent_impl]
    #[unstable(feature = "f16", issue = "116909")]
    // #[unstable(feature = "float_gamma", issue = "99842")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln_gamma(self) -> (f16, i32) {
        let mut signgamp: i32 = 0;
        let x = cmath::lgammaf_r(self as f32, &mut signgamp) as f16;
        (x, signgamp)
    }

    /// Error function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `erff` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// #![feature(float_erf)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    /// /// The error function relates what percent of a normal distribution lies
    /// /// within `x` standard deviations (scaled by `1/sqrt(2)`).
    /// fn within_standard_deviations(x: f16) -> f16 {
    ///     (x * std::f16::consts::FRAC_1_SQRT_2).erf() * 100.0
    /// }
    ///
    /// // 68% of a normal distribution is within one standard deviation
    /// assert!((within_standard_deviations(1.0) - 68.269).abs() < 0.1);
    /// // 95% of a normal distribution is within two standard deviations
    /// assert!((within_standard_deviations(2.0) - 95.450).abs() < 0.1);
    /// // 99.7% of a normal distribution is within three standard deviations
    /// assert!((within_standard_deviations(3.0) - 99.730).abs() < 0.1);
    /// # }
    /// ```
    #[rustc_allow_incoherent_impl]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[unstable(feature = "f16", issue = "116909")]
    // #[unstable(feature = "float_erf", issue = "136321")]
    #[inline]
    pub fn erf(self) -> f16 {
        cmath::erff(self as f32) as f16
    }

    /// Complementary error function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform,
    /// Rust version, and can even differ within the same execution from one invocation to the next.
    ///
    /// This function currently corresponds to the `erfcf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(f16)]
    /// #![feature(float_erf)]
    /// # #[cfg(not(miri))]
    /// # #[cfg(target_has_reliable_f16_math)] {
    /// let x: f16 = 0.123;
    ///
    /// let one = x.erf() + x.erfc();
    /// let abs_difference = (one - 1.0).abs();
    ///
    /// assert!(abs_difference <= f16::EPSILON);
    /// # }
    /// ```
    #[rustc_allow_incoherent_impl]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[unstable(feature = "f16", issue = "116909")]
    // #[unstable(feature = "float_erf", issue = "136321")]
    #[inline]
    pub fn erfc(self) -> f16 {
        cmath::erfcf(self as f32) as f16
    }
}
