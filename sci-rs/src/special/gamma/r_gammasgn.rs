use num_traits::{cast, Float};
use crate::special::gamma_util::is_gamma_pole;

pub(crate) fn r_gammasgn<Real>(x: Real) -> Real
where Real: Float {
    if !x.is_finite() {
        return x;
    } 
    if is_gamma_pole(x) {
        return Real::zero();
    }
    if x.is_sign_positive() {
        return Real::one();
    }
    if cast::<Real, usize>(x.abs().floor()).unwrap() & 1 == 1 {
        Real::one()
    } else {
        -Real::one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gammasgn() {
        for i in 1..20 {
            assert_eq!(r_gammasgn(i as f32), 1.0);
            assert_eq!(r_gammasgn(i as f32 + 0.5), 1.0);
            assert_eq!(r_gammasgn(i as f32 + 0.25), 1.0);

            let neg_even = -(2 * i) as f32;
            assert_eq!(r_gammasgn(neg_even - 0.5), -1.0);
            assert_eq!(r_gammasgn(neg_even - 0.25), -1.0);

            let neg_odd = -(2 * i + 1) as f32;
            assert_eq!(r_gammasgn(neg_odd - 0.5), 1.0);
            assert_eq!(r_gammasgn(neg_odd - 0.25), 1.0);
        }
    }
}
