//**********************************************************************
// This file is part of sci-rs                                         *
// Copyright 2023 Matthew R. Hennefarth                                *
//**********************************************************************

use crate::special::gamma::gamma_util::is_gamma_pole;
use crate::special::gamma::real_gamma_impl::*;
use crate::traits::FloatSciConst;
use num_traits::{cast, Float};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub(crate) trait RealPochConsts {
    const MIN_FOR_EXP: Self;
}

macro_rules! impl_realpochconsts {
    ($($T: ty)*) => ($(
        impl RealPochConsts for $T {
            const MIN_FOR_EXP: Self = 1.0e4;
        }
)*)
}

impl_realpochconsts! {f32 f64}

/// Implementation of the Pochhammer symbols for real-valued arguments
/// $$
/// x^{(m)} = \frac{\Gamma(x+m)}{\Gamma(x)}
/// $$
/// where both $x$ and $m$ are real-valued.
pub(crate) fn r_poch<T>(x: T, mut m: T) -> T
where
    T: Float
        + SubAssign
        + AddAssign
        + MulAssign
        + DivAssign
        + RealPochConsts
        + FloatSciConst
        + RealGammaLnConsts,
{
    let mut r = T::one();

    while m >= T::one() {
        if x + m == T::one() {
            break;
        }
        m -= T::one();
        r *= x + m;
        if !r.is_finite() || r.is_zero() {
            break;
        }
    }

    while m <= -T::one() {
        if x + m == T::one() {
            break;
        }
        r /= x + m;
        m += T::one();
        if !r.is_finite() || r.is_zero() {
            break;
        }
    }

    if m.is_zero() {
        return r;
    }
    if x > T::MIN_FOR_EXP && m.abs() <= T::one() {
        let two = T::one() + T::one();
        let three = two + T::one();
        return r
            * x.powf(m)
            * (T::one()
                + m * (m - T::one()) / (two * x)
                + m * (m - T::one()) * (m - two) * (three * m - T::one())
                    / (cast::<u8, T>(24).unwrap() * x * x)
                + m * m * (m - T::one()) * (m - T::one()) * (m - two) * (m - three)
                    / (cast::<u8, T>(48).unwrap() * x * x * x));
    }

    // Check for infinite
    if is_gamma_pole(x + m) && !is_gamma_pole(x) && x + m != m {
        return T::nan();
    }

    // Check for zero
    if !is_gamma_pole(x + m) && is_gamma_pole(x) {
        return T::zero();
    }

    r * (r_lgamma(x + m) - r_lgamma(x)).exp() * r_gammasgn(x + m) * r_gammasgn(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::f64::PI;
    use crate::special::Factorial;

    const PRECISION: f64 = 1e-14;

    #[test]
    fn test_rpoch() {
        for i in 1..10 {
            for m in 0..5 {
                assert_eq!(
                    r_poch(i as f64, m as f64),
                    (i + m - 1).factorial() as f64 / (i - 1).factorial() as f64
                );
            }
        }

        const ABSOLUTE_KNOWN_VALUES: [[f64; 3]; 5] = [
            // Weird edge values
            [0.0, 0.0, 1.0],
            [0.0, 0.25, 0.0],
            [-1.0, 0.0, 1.0],
            [-1.0, 0.25, 0.0],
            // This feels wrong since this is gamma(-1)/gamma(-2) which should be undefined. But technically you can use the rules of the gamma function to write as -2 gamma(-2)/gamma(-2). So I am not sure how I feel about it. But it agrees with SciPy currently.
            [-2.0, 1.0, -2.0],
        ];

        const KNOWN_VALUES: [[f64; 3]; 12] = [
            [-2.2, 0.2, f64::NAN],
            // Following values taken from SciPy version 1.10.1
            [0.5, 0.5, 0.56418958354775639030],
            [0.5, 1.0, 0.5],
            [1.0, 0.5, 0.88622692545275794096],
            [1.5, 1.0, 1.5],
            [1.0, 1.5, 1.32934038817913702246],
            [1.5, 1.5, 2.25675833419102511712],
            [2.5, 2.5, 18.05406667352819738426],
            [150.00001, PI, 7015772.59900571219623088837],
            // Add some larger values to compare to!
            // From Wolframalpha
            [PI / 2.0, PI, 17.63940522158362397144],
            [-34.54, -2.4, 0.000959271152790991576995792318463],
            [-1.5, -0.22, 1.099148632503722270901806],
        ];

        for value in ABSOLUTE_KNOWN_VALUES {
            assert_eq!(r_poch(value[0], value[1]), value[2]);
        }

        for value in KNOWN_VALUES {
            assert_almost_eq!(r_poch(value[0], value[1]), value[2], PRECISION);
        }
    }
}
