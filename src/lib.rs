// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Integer trait and functions.

#![doc(html_root_url = "https://docs.rs/num-integer/0.1")]

extern crate num_traits as traits;

use std::ops::Add;

use traits::{Num, Signed};

pub trait Integer: Sized + Num + PartialOrd + Ord + Eq {
    /// Floored integer division.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert!(( 8).div_floor(& 3) ==  2);
    /// assert!(( 8).div_floor(&-3) == -3);
    /// assert!((-8).div_floor(& 3) == -3);
    /// assert!((-8).div_floor(&-3) ==  2);
    ///
    /// assert!(( 1).div_floor(& 2) ==  0);
    /// assert!(( 1).div_floor(&-2) == -1);
    /// assert!((-1).div_floor(& 2) == -1);
    /// assert!((-1).div_floor(&-2) ==  0);
    /// ~~~
    fn div_floor(&self, other: &Self) -> Self;

    /// Floored integer modulo, satisfying:
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// # let n = 1; let d = 1;
    /// assert!(n.div_floor(&d) * d + n.mod_floor(&d) == n)
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert!(( 8).mod_floor(& 3) ==  2);
    /// assert!(( 8).mod_floor(&-3) == -1);
    /// assert!((-8).mod_floor(& 3) ==  1);
    /// assert!((-8).mod_floor(&-3) == -2);
    ///
    /// assert!(( 1).mod_floor(& 2) ==  1);
    /// assert!(( 1).mod_floor(&-2) == -1);
    /// assert!((-1).mod_floor(& 2) ==  1);
    /// assert!((-1).mod_floor(&-2) == -1);
    /// ~~~
    fn mod_floor(&self, other: &Self) -> Self;

    /// Greatest Common Divisor (GCD).
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(6.gcd(&8), 2);
    /// assert_eq!(7.gcd(&3), 1);
    /// ~~~
    fn gcd(&self, other: &Self) -> Self;

    /// Lowest Common Multiple (LCM).
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(7.lcm(&3), 21);
    /// assert_eq!(2.lcm(&4), 4);
    /// ~~~
    fn lcm(&self, other: &Self) -> Self;

    /// Deprecated, use `is_multiple_of` instead.
    fn divides(&self, other: &Self) -> bool;

    /// Returns `true` if `other` is a multiple of `self`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(9.is_multiple_of(&3), true);
    /// assert_eq!(3.is_multiple_of(&9), false);
    /// ~~~
    fn is_multiple_of(&self, other: &Self) -> bool;

    /// Returns `true` if the number is even.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(3.is_even(), false);
    /// assert_eq!(4.is_even(), true);
    /// ~~~
    fn is_even(&self) -> bool;

    /// Returns `true` if the number is odd.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(3.is_odd(), true);
    /// assert_eq!(4.is_odd(), false);
    /// ~~~
    fn is_odd(&self) -> bool;

    /// Simultaneous truncated integer division and modulus.
    /// Returns `(quotient, remainder)`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(( 8).div_rem( &3), ( 2,  2));
    /// assert_eq!(( 8).div_rem(&-3), (-2,  2));
    /// assert_eq!((-8).div_rem( &3), (-2, -2));
    /// assert_eq!((-8).div_rem(&-3), ( 2, -2));
    ///
    /// assert_eq!(( 1).div_rem( &2), ( 0,  1));
    /// assert_eq!(( 1).div_rem(&-2), ( 0,  1));
    /// assert_eq!((-1).div_rem( &2), ( 0, -1));
    /// assert_eq!((-1).div_rem(&-2), ( 0, -1));
    /// ~~~
    #[inline]
    fn div_rem(&self, other: &Self) -> (Self, Self);

    /// Simultaneous floored integer division and modulus.
    /// Returns `(quotient, remainder)`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use num_integer::Integer;
    /// assert_eq!(( 8).div_mod_floor( &3), ( 2,  2));
    /// assert_eq!(( 8).div_mod_floor(&-3), (-3, -1));
    /// assert_eq!((-8).div_mod_floor( &3), (-3,  1));
    /// assert_eq!((-8).div_mod_floor(&-3), ( 2, -2));
    ///
    /// assert_eq!(( 1).div_mod_floor( &2), ( 0,  1));
    /// assert_eq!(( 1).div_mod_floor(&-2), (-1, -1));
    /// assert_eq!((-1).div_mod_floor( &2), (-1,  1));
    /// assert_eq!((-1).div_mod_floor(&-2), ( 0, -1));
    /// ~~~
    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        (self.div_floor(other), self.mod_floor(other))
    }
}

/// Simultaneous integer division and modulus
#[inline]
pub fn div_rem<T: Integer>(x: T, y: T) -> (T, T) {
    x.div_rem(&y)
}
/// Floored integer division
#[inline]
pub fn div_floor<T: Integer>(x: T, y: T) -> T {
    x.div_floor(&y)
}
/// Floored integer modulus
#[inline]
pub fn mod_floor<T: Integer>(x: T, y: T) -> T {
    x.mod_floor(&y)
}
/// Simultaneous floored integer division and modulus
#[inline]
pub fn div_mod_floor<T: Integer>(x: T, y: T) -> (T, T) {
    x.div_mod_floor(&y)
}

/// Calculates the Greatest Common Divisor (GCD) of the number and `other`. The
/// result is always positive.
#[inline(always)]
pub fn gcd<T: Integer>(x: T, y: T) -> T {
    x.gcd(&y)
}
/// Calculates the Lowest Common Multiple (LCM) of the number and `other`.
#[inline(always)]
pub fn lcm<T: Integer>(x: T, y: T) -> T {
    x.lcm(&y)
}

macro_rules! impl_integer_for_isize {
    ($T:ty, $test_mod:ident) => (
        impl Integer for $T {
            /// Floored integer division
            #[inline]
            fn div_floor(&self, other: &Self) -> Self {
                // Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
                // December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)
                match self.div_rem(other) {
                    (d, r) if (r > 0 && *other < 0)
                           || (r < 0 && *other > 0) => d - 1,
                    (d, _)                          => d,
                }
            }

            /// Floored integer modulo
            #[inline]
            fn mod_floor(&self, other: &Self) -> Self {
                // Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
                // December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)
                match *self % *other {
                    r if (r > 0 && *other < 0)
                      || (r < 0 && *other > 0) => r + *other,
                    r                          => r,
                }
            }

            /// Calculates `div_floor` and `mod_floor` simultaneously
            #[inline]
            fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
                // Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
                // December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)
                match self.div_rem(other) {
                    (d, r) if (r > 0 && *other < 0)
                           || (r < 0 && *other > 0) => (d - 1, r + *other),
                    (d, r)                          => (d, r),
                }
            }

            /// Calculates the Greatest Common Divisor (GCD) of the number and
            /// `other`. The result is always positive.
            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                // Use Stein's algorithm
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 { return (m | n).abs() }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // The algorithm needs positive numbers, but the minimum value
                // can't be represented as a positive one.
                // It's also a power of two, so the gcd can be
                // calculated by bitshifting in that case

                // Assuming two's complement, the number created by the shift
                // is positive for all numbers except gcd = abs(min value)
                // The call to .abs() causes a panic in debug mode
                if m == Self::min_value() || n == Self::min_value() {
                    return (1 << shift).abs()
                }

                // guaranteed to be positive now, rest like unsigned algorithm
                m = m.abs();
                n = n.abs();

                // divide n and m by 2 until odd
                // m inside loop
                n >>= n.trailing_zeros();

                while m != 0 {
                    m >>= m.trailing_zeros();
                    if n > m { ::std::mem::swap(&mut n, &mut m) }
                    m -= n;
                }

                n << shift
            }

            /// Calculates the Lowest Common Multiple (LCM) of the number and
            /// `other`.
            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                // should not have to recalculate abs
                (*self * (*other / self.gcd(other))).abs()
            }

            /// Deprecated, use `is_multiple_of` instead.
            #[inline]
            fn divides(&self, other: &Self) -> bool {
                self.is_multiple_of(other)
            }

            /// Returns `true` if the number is a multiple of `other`.
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                *self % *other == 0
            }

            /// Returns `true` if the number is divisible by `2`
            #[inline]
            fn is_even(&self) -> bool { (*self) & 1 == 0 }

            /// Returns `true` if the number is not divisible by `2`
            #[inline]
            fn is_odd(&self) -> bool { !self.is_even() }

            /// Simultaneous truncated integer division and modulus.
            #[inline]
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                (*self / *other, *self % *other)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use Integer;

            /// Checks that the division rule holds for:
            ///
            /// - `n`: numerator (dividend)
            /// - `d`: denominator (divisor)
            /// - `qr`: quotient and remainder
            #[cfg(test)]
            fn test_division_rule((n,d): ($T, $T), (q,r): ($T, $T)) {
                assert_eq!(d * q + r, n);
            }

            #[test]
            fn test_div_rem() {
                fn test_nd_dr(nd: ($T,$T), qr: ($T,$T)) {
                    let (n,d) = nd;
                    let separate_div_rem = (n / d, n % d);
                    let combined_div_rem = n.div_rem(&d);

                    assert_eq!(separate_div_rem, qr);
                    assert_eq!(combined_div_rem, qr);

                    test_division_rule(nd, separate_div_rem);
                    test_division_rule(nd, combined_div_rem);
                }

                test_nd_dr(( 8,  3), ( 2,  2));
                test_nd_dr(( 8, -3), (-2,  2));
                test_nd_dr((-8,  3), (-2, -2));
                test_nd_dr((-8, -3), ( 2, -2));

                test_nd_dr(( 1,  2), ( 0,  1));
                test_nd_dr(( 1, -2), ( 0,  1));
                test_nd_dr((-1,  2), ( 0, -1));
                test_nd_dr((-1, -2), ( 0, -1));
            }

            #[test]
            fn test_div_mod_floor() {
                fn test_nd_dm(nd: ($T,$T), dm: ($T,$T)) {
                    let (n,d) = nd;
                    let separate_div_mod_floor = (n.div_floor(&d), n.mod_floor(&d));
                    let combined_div_mod_floor = n.div_mod_floor(&d);

                    assert_eq!(separate_div_mod_floor, dm);
                    assert_eq!(combined_div_mod_floor, dm);

                    test_division_rule(nd, separate_div_mod_floor);
                    test_division_rule(nd, combined_div_mod_floor);
                }

                test_nd_dm(( 8,  3), ( 2,  2));
                test_nd_dm(( 8, -3), (-3, -1));
                test_nd_dm((-8,  3), (-3,  1));
                test_nd_dm((-8, -3), ( 2, -2));

                test_nd_dm(( 1,  2), ( 0,  1));
                test_nd_dm(( 1, -2), (-1, -1));
                test_nd_dm((-1,  2), (-1,  1));
                test_nd_dm((-1, -2), ( 0, -1));
            }

            #[test]
            fn test_gcd() {
                assert_eq!((10 as $T).gcd(&2), 2 as $T);
                assert_eq!((10 as $T).gcd(&3), 1 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
                assert_eq!((56 as $T).gcd(&42), 14 as $T);
                assert_eq!((3 as $T).gcd(&-3), 3 as $T);
                assert_eq!((-6 as $T).gcd(&3), 3 as $T);
                assert_eq!((-4 as $T).gcd(&-2), 2 as $T);
            }

            #[test]
            fn test_gcd_cmp_with_euclidean() {
                fn euclidean_gcd(mut m: $T, mut n: $T) -> $T {
                    while m != 0 {
                        ::std::mem::swap(&mut m, &mut n);
                        m %= n;
                    }

                    n.abs()
                }

                // gcd(-128, b) = 128 is not representable as positive value
                // for i8
                for i in -127..127 {
                    for j in -127..127 {
                        assert_eq!(euclidean_gcd(i,j), i.gcd(&j));
                    }
                }

                // last value
                // FIXME: Use inclusive ranges for above loop when implemented
                let i = 127;
                for j in -127..127 {
                    assert_eq!(euclidean_gcd(i,j), i.gcd(&j));
                }
                assert_eq!(127.gcd(&127), 127);
            }

            #[test]
            fn test_gcd_min_val() {
                let min = <$T>::min_value();
                let max = <$T>::max_value();
                let max_pow2 = max / 2 + 1;
                assert_eq!(min.gcd(&max), 1 as $T);
                assert_eq!(max.gcd(&min), 1 as $T);
                assert_eq!(min.gcd(&max_pow2), max_pow2);
                assert_eq!(max_pow2.gcd(&min), max_pow2);
                assert_eq!(min.gcd(&42), 2 as $T);
                assert_eq!((42 as $T).gcd(&min), 2 as $T);
            }

            #[test]
            #[should_panic]
            fn test_gcd_min_val_min_val() {
                let min = <$T>::min_value();
                assert!(min.gcd(&min) >= 0);
            }

            #[test]
            #[should_panic]
            fn test_gcd_min_val_0() {
                let min = <$T>::min_value();
                assert!(min.gcd(&0) >= 0);
            }

            #[test]
            #[should_panic]
            fn test_gcd_0_min_val() {
                let min = <$T>::min_value();
                assert!((0 as $T).gcd(&min) >= 0);
            }

            #[test]
            fn test_lcm() {
                assert_eq!((1 as $T).lcm(&0), 0 as $T);
                assert_eq!((0 as $T).lcm(&1), 0 as $T);
                assert_eq!((1 as $T).lcm(&1), 1 as $T);
                assert_eq!((-1 as $T).lcm(&1), 1 as $T);
                assert_eq!((1 as $T).lcm(&-1), 1 as $T);
                assert_eq!((-1 as $T).lcm(&-1), 1 as $T);
                assert_eq!((8 as $T).lcm(&9), 72 as $T);
                assert_eq!((11 as $T).lcm(&5), 55 as $T);
            }

            #[test]
            fn test_even() {
                assert_eq!((-4 as $T).is_even(), true);
                assert_eq!((-3 as $T).is_even(), false);
                assert_eq!((-2 as $T).is_even(), true);
                assert_eq!((-1 as $T).is_even(), false);
                assert_eq!((0 as $T).is_even(), true);
                assert_eq!((1 as $T).is_even(), false);
                assert_eq!((2 as $T).is_even(), true);
                assert_eq!((3 as $T).is_even(), false);
                assert_eq!((4 as $T).is_even(), true);
            }

            #[test]
            fn test_odd() {
                assert_eq!((-4 as $T).is_odd(), false);
                assert_eq!((-3 as $T).is_odd(), true);
                assert_eq!((-2 as $T).is_odd(), false);
                assert_eq!((-1 as $T).is_odd(), true);
                assert_eq!((0 as $T).is_odd(), false);
                assert_eq!((1 as $T).is_odd(), true);
                assert_eq!((2 as $T).is_odd(), false);
                assert_eq!((3 as $T).is_odd(), true);
                assert_eq!((4 as $T).is_odd(), false);
            }
        }
    )
}

impl_integer_for_isize!(i8, test_integer_i8);
impl_integer_for_isize!(i16, test_integer_i16);
impl_integer_for_isize!(i32, test_integer_i32);
impl_integer_for_isize!(i64, test_integer_i64);
impl_integer_for_isize!(isize, test_integer_isize);

macro_rules! impl_integer_for_usize {
    ($T:ty, $test_mod:ident) => (
        impl Integer for $T {
            /// Unsigned integer division. Returns the same result as `div` (`/`).
            #[inline]
            fn div_floor(&self, other: &Self) -> Self {
                *self / *other
            }

            /// Unsigned integer modulo operation. Returns the same result as `rem` (`%`).
            #[inline]
            fn mod_floor(&self, other: &Self) -> Self {
                *self % *other
            }

            /// Calculates the Greatest Common Divisor (GCD) of the number and `other`
            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                // Use Stein's algorithm
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 { return m | n }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // divide n and m by 2 until odd
                // m inside loop
                n >>= n.trailing_zeros();

                while m != 0 {
                    m >>= m.trailing_zeros();
                    if n > m { ::std::mem::swap(&mut n, &mut m) }
                    m -= n;
                }

                n << shift
            }

            /// Calculates the Lowest Common Multiple (LCM) of the number and `other`.
            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                *self * (*other / self.gcd(other))
            }

            /// Deprecated, use `is_multiple_of` instead.
            #[inline]
            fn divides(&self, other: &Self) -> bool {
                self.is_multiple_of(other)
            }

            /// Returns `true` if the number is a multiple of `other`.
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                *self % *other == 0
            }

            /// Returns `true` if the number is divisible by `2`.
            #[inline]
            fn is_even(&self) -> bool {
                *self % 2 == 0
            }

            /// Returns `true` if the number is not divisible by `2`.
            #[inline]
            fn is_odd(&self) -> bool {
                !self.is_even()
            }

            /// Simultaneous truncated integer division and modulus.
            #[inline]
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                (*self / *other, *self % *other)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use Integer;

            #[test]
            fn test_div_mod_floor() {
                assert_eq!((10 as $T).div_floor(&(3 as $T)), 3 as $T);
                assert_eq!((10 as $T).mod_floor(&(3 as $T)), 1 as $T);
                assert_eq!((10 as $T).div_mod_floor(&(3 as $T)), (3 as $T, 1 as $T));
                assert_eq!((5 as $T).div_floor(&(5 as $T)), 1 as $T);
                assert_eq!((5 as $T).mod_floor(&(5 as $T)), 0 as $T);
                assert_eq!((5 as $T).div_mod_floor(&(5 as $T)), (1 as $T, 0 as $T));
                assert_eq!((3 as $T).div_floor(&(7 as $T)), 0 as $T);
                assert_eq!((3 as $T).mod_floor(&(7 as $T)), 3 as $T);
                assert_eq!((3 as $T).div_mod_floor(&(7 as $T)), (0 as $T, 3 as $T));
            }

            #[test]
            fn test_gcd() {
                assert_eq!((10 as $T).gcd(&2), 2 as $T);
                assert_eq!((10 as $T).gcd(&3), 1 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
                assert_eq!((56 as $T).gcd(&42), 14 as $T);
            }

            #[test]
            fn test_gcd_cmp_with_euclidean() {
                fn euclidean_gcd(mut m: $T, mut n: $T) -> $T {
                    while m != 0 {
                        ::std::mem::swap(&mut m, &mut n);
                        m %= n;
                    }
                    n
                }

                for i in 0..255 {
                    for j in 0..255 {
                        assert_eq!(euclidean_gcd(i,j), i.gcd(&j));
                    }
                }

                // last value
                // FIXME: Use inclusive ranges for above loop when implemented
                let i = 255;
                for j in 0..255 {
                    assert_eq!(euclidean_gcd(i,j), i.gcd(&j));
                }
                assert_eq!(255.gcd(&255), 255);
            }

            #[test]
            fn test_lcm() {
                assert_eq!((1 as $T).lcm(&0), 0 as $T);
                assert_eq!((0 as $T).lcm(&1), 0 as $T);
                assert_eq!((1 as $T).lcm(&1), 1 as $T);
                assert_eq!((8 as $T).lcm(&9), 72 as $T);
                assert_eq!((11 as $T).lcm(&5), 55 as $T);
                assert_eq!((15 as $T).lcm(&17), 255 as $T);
            }

            #[test]
            fn test_is_multiple_of() {
                assert!((6 as $T).is_multiple_of(&(6 as $T)));
                assert!((6 as $T).is_multiple_of(&(3 as $T)));
                assert!((6 as $T).is_multiple_of(&(1 as $T)));
            }

            #[test]
            fn test_even() {
                assert_eq!((0 as $T).is_even(), true);
                assert_eq!((1 as $T).is_even(), false);
                assert_eq!((2 as $T).is_even(), true);
                assert_eq!((3 as $T).is_even(), false);
                assert_eq!((4 as $T).is_even(), true);
            }

            #[test]
            fn test_odd() {
                assert_eq!((0 as $T).is_odd(), false);
                assert_eq!((1 as $T).is_odd(), true);
                assert_eq!((2 as $T).is_odd(), false);
                assert_eq!((3 as $T).is_odd(), true);
                assert_eq!((4 as $T).is_odd(), false);
            }
        }
    )
}

impl_integer_for_usize!(u8, test_integer_u8);
impl_integer_for_usize!(u16, test_integer_u16);
impl_integer_for_usize!(u32, test_integer_u32);
impl_integer_for_usize!(u64, test_integer_u64);
impl_integer_for_usize!(usize, test_integer_usize);

/// An iterator over binomial coefficients.
pub struct IterBinomial<T> {
    a: T,
    n: T,
    k: T,
}

impl<T> IterBinomial<T>
    where T: Integer,
{
    /// For a given n, iterate over all binomial coefficients binomial(n, k), for k=0...n.
    ///
    /// Note that this might overflow, depending on `T`. For the primitive
    /// integer types, the following n are the largest ones for which there will
    /// be no overflow:
    ///
    /// type | n
    /// -----|---
    /// u8   | 10
    /// i8   |  9
    /// u16  | 18
    /// i16  | 17
    /// u32  | 34
    /// i32  | 33
    /// u64  | 67
    /// i64  | 66
    ///
    /// For larger n, `T` should be a bigint type.
    pub fn new(n: T) -> IterBinomial<T> {
        IterBinomial {
            k: T::zero(), a: T::one(), n: n
        }
    }
}

impl<T> Iterator for IterBinomial<T>
    where T: Integer + Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.k > self.n {
            return None;
        }
        self.a = if !self.k.is_zero() {
            multiply_and_divide(
                self.a.clone(),
                self.n.clone() - self.k.clone() + T::one(),
                self.k.clone()
            )
        } else {
            T::one()
        };
        self.k = self.k.clone() + T::one();
        Some(self.a.clone())
    }
}

/// Calculate r * a / b, avoiding overflows and fractions.
///
/// Assumes that b divides r * a evenly.
fn multiply_and_divide<T: Integer + Clone>(r: T, a: T, b: T) -> T {
    // See http://blog.plover.com/math/choose-2.html for the idea.
    let g = gcd(r.clone(), b.clone());
    r/g.clone() * (a / (b/g))
}

/// Calculate the binomial coefficient.
///
/// Note that this might overflow, depending on `T`. For the primitive integer
/// types, the following n are the largest ones possible such that there will
/// be no overflow for any k:
///
/// type | n
/// -----|---
/// u8   | 10
/// i8   |  9
/// u16  | 18
/// i16  | 17
/// u32  | 34
/// i32  | 33
/// u64  | 67
/// i64  | 66
///
/// For larger n, consider using a bigint type for `T`.
pub fn binomial<T: Integer + Clone>(mut n: T, k: T) -> T {
    // See http://blog.plover.com/math/choose.html for the idea.
    if k > n {
        return T::zero();
    }
    if k > n.clone() - k.clone() {
        return binomial(n.clone(), n - k);
    }
    let mut r = T::one();
    let mut d = T::one();
    loop {
        if d > k {
            break;
        }
        r = multiply_and_divide(r, n.clone(), d.clone());
        n = n - T::one();
        d = d + T::one();
    }
    r
}

/// Calculate the multinomial coefficient.
pub fn multinomial<T: Integer + Clone>(k: &[T]) -> T
    where for<'a> T: Add<&'a T, Output = T>
{
    let mut r = T::one();
    let mut p = T::zero();
    for i in k {
        p = p + i;
        r = r * binomial(p.clone(), i.clone());
    }
    r
}

#[test]
fn test_lcm_overflow() {
    macro_rules! check {
        ($t:ty, $x:expr, $y:expr, $r:expr) => { {
            let x: $t = $x;
            let y: $t = $y;
            let o = x.checked_mul(y);
            assert!(o.is_none(),
                    "sanity checking that {} input {} * {} overflows",
                    stringify!($t), x, y);
            assert_eq!(x.lcm(&y), $r);
            assert_eq!(y.lcm(&x), $r);
        } }
    }

    // Original bug (Issue #166)
    check!(i64, 46656000000000000, 600, 46656000000000000);

    check!(i8, 0x40, 0x04, 0x40);
    check!(u8, 0x80, 0x02, 0x80);
    check!(i16, 0x40_00, 0x04, 0x40_00);
    check!(u16, 0x80_00, 0x02, 0x80_00);
    check!(i32, 0x4000_0000, 0x04, 0x4000_0000);
    check!(u32, 0x8000_0000, 0x02, 0x8000_0000);
    check!(i64, 0x4000_0000_0000_0000, 0x04, 0x4000_0000_0000_0000);
    check!(u64, 0x8000_0000_0000_0000, 0x02, 0x8000_0000_0000_0000);
}

#[test]
fn test_iter_binomial() {
    macro_rules! check_simple {
        ($t:ty) => { {
            let n: $t = 3;
            let c: Vec<_> = IterBinomial::new(n).collect();
            let expected = vec![1, 3, 3, 1];
            assert_eq!(c, expected);
        } }
    }

    check_simple!(u8);
    check_simple!(i8);
    check_simple!(u16);
    check_simple!(i16);
    check_simple!(u32);
    check_simple!(i32);
    check_simple!(u64);
    check_simple!(i64);

    macro_rules! check_binomial {
        ($t:ty, $n:expr) => { {
            let n: $t = $n;
            let c: Vec<_> = IterBinomial::new(n).collect();
            let mut k: $t = 0;
            for b in c {
                assert_eq!(b, binomial(n, k));
                k += 1;
            }
        } }
    }

    // Check the largest n for which there is no overflow.
    check_binomial!(u8, 10);
    check_binomial!(i8, 9);
    check_binomial!(u16, 18);
    check_binomial!(i16, 17);
    check_binomial!(u32, 34);
    check_binomial!(i32, 33);
    check_binomial!(u64, 67);
    check_binomial!(i64, 66);
}

#[test]
fn test_binomial() {
    macro_rules! check {
        ($t:ty, $x:expr, $y:expr, $r:expr) => { {
            let x: $t = $x;
            let y: $t = $y;
            let expected: $t = $r;
            assert_eq!(binomial(x, y), expected);
            if y <= x {
                assert_eq!(binomial(x, x - y), expected);
            }
        } }
    }
    check!(u8, 9, 4, 126);
    check!(u8, 0, 0, 1);
    check!(u8, 2, 3, 0);

    check!(i8, 9, 4, 126);
    check!(i8, 0, 0, 1);
    check!(i8, 2, 3, 0);

    check!(u16, 100, 2, 4950);
    check!(u16, 14, 4, 1001);
    check!(u16, 0, 0, 1);
    check!(u16, 2, 3, 0);

    check!(i16, 100, 2, 4950);
    check!(i16, 14, 4, 1001);
    check!(i16, 0, 0, 1);
    check!(i16, 2, 3, 0);

    check!(u32, 100, 2, 4950);
    check!(u32, 35, 11, 417225900);
    check!(u32, 14, 4, 1001);
    check!(u32, 0, 0, 1);
    check!(u32, 2, 3, 0);

    check!(i32, 100, 2, 4950);
    check!(i32, 35, 11, 417225900);
    check!(i32, 14, 4, 1001);
    check!(i32, 0, 0, 1);
    check!(i32, 2, 3, 0);

    check!(u64, 100, 2, 4950);
    check!(u64, 35, 11, 417225900);
    check!(u64, 14, 4, 1001);
    check!(u64, 0, 0, 1);
    check!(u64, 2, 3, 0);

    check!(i64, 100, 2, 4950);
    check!(i64, 35, 11, 417225900);
    check!(i64, 14, 4, 1001);
    check!(i64, 0, 0, 1);
    check!(i64, 2, 3, 0);
}

#[test]
fn test_multinomial() {
    macro_rules! check_binomial {
        ($t:ty, $k:expr) => { {
            let n: $t = $k.iter().fold(0, |acc, &x| acc + x);
            let k: &[$t] = $k;
            assert_eq!(k.len(), 2);
            assert_eq!(multinomial(k), binomial(n, k[0]));
        } }
    }

    check_binomial!(u8, &[4, 5]);

    check_binomial!(i8, &[4, 5]);

    check_binomial!(u16, &[2, 98]);
    check_binomial!(u16, &[4, 10]);

    check_binomial!(i16, &[2, 98]);
    check_binomial!(i16, &[4, 10]);

    check_binomial!(u32, &[2, 98]);
    check_binomial!(u32, &[11, 24]);
    check_binomial!(u32, &[4, 10]);

    check_binomial!(i32, &[2, 98]);
    check_binomial!(i32, &[11, 24]);
    check_binomial!(i32, &[4, 10]);

    check_binomial!(u64, &[2, 98]);
    check_binomial!(u64, &[11, 24]);
    check_binomial!(u64, &[4, 10]);

    check_binomial!(i64, &[2, 98]);
    check_binomial!(i64, &[11, 24]);
    check_binomial!(i64, &[4, 10]);

    macro_rules! check_multinomial {
        ($t:ty, $k:expr, $r:expr) => { {
            let k: &[$t] = $k;
            let expected: $t = $r;
            assert_eq!(multinomial(k), expected);
        } }
    }

    check_multinomial!(u8, &[2, 1, 2], 30);
    check_multinomial!(u8, &[2, 3, 0], 10);

    check_multinomial!(i8, &[2, 1, 2], 30);
    check_multinomial!(i8, &[2, 3, 0], 10);

    check_multinomial!(u16, &[2, 1, 2], 30);
    check_multinomial!(u16, &[2, 3, 0], 10);

    check_multinomial!(i16, &[2, 1, 2], 30);
    check_multinomial!(i16, &[2, 3, 0], 10);

    check_multinomial!(u32, &[2, 1, 2], 30);
    check_multinomial!(u32, &[2, 3, 0], 10);

    check_multinomial!(i32, &[2, 1, 2], 30);
    check_multinomial!(i32, &[2, 3, 0], 10);

    check_multinomial!(u64, &[2, 1, 2], 30);
    check_multinomial!(u64, &[2, 3, 0], 10);

    check_multinomial!(i64, &[2, 1, 2], 30);
    check_multinomial!(i64, &[2, 3, 0], 10);

    check_multinomial!(u64, &[], 1);
    check_multinomial!(u64, &[0], 1);
    check_multinomial!(u64, &[12345], 1);
}
