use num_traits::{Float};

/// Determins if `x` is at a pole of the Gamma function (0, -1, -2, etc).
#[inline]
pub(crate) fn is_gamma_pole<Real>(x: Real) -> bool
where
    Real: Float
{
    // TODO: generalize this to Complex values once a complex package is decided on
    x <= Real::zero() && x == x.floor() // and z.im().is_zero()
}
