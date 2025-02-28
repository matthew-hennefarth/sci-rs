
use crate::special::gamma::real_gamma_impl::*; 

/// Gamma and related functions for both real and complex-valued valued arguments.
///
/// # Implementation Notes
/// For most implementations, a few properties are exploited to simplify the approximations. Firstly, the reflection property is used to always translate the value to the positive real axis.
/// $$
/// \Gamma(-z)\Gamma(z) = -\frac{\pi}{z\sin(\pi z)}
/// $$
/// Additionally, the recursive nature of the Gamma function is often used to move the value into some desired region to then approximate.
/// $$
/// \Gamma(z+1) = z\Gamma(z)
/// $$
pub trait Gamma{

}

/// Gamma related functions which only make sense, or are only currently supported for real-valued arguments.
pub trait RealGamma: Gamma {
    /// Sign of the [gamma] function.
    ///
    /// $$
    /// \text{gammasgn}(x) = \begin{cases}
    /// +1.0 & \Gamma(x) > 0 \\\\
    /// -1.0 & \Gamma(x) < 0
    /// \end{cases}
    /// $$
    /// The [gamma] function, for real-valued arguments $x$, is never zero and so this is a well-defined function.
    ///
    /// # Examples
    /// ```
    /// use sci_rs::special::RealGamma;
    /// assert_eq!(1.23.gammasgn(), 1.0);
    /// assert_eq!((-0.23).gammasgn(), -1.0);
    /// assert_eq!((-1.5).gammasgn(), 1.0);
    /// ```
    /// # Notes
    /// We return `0.0` if $\Gamma(x)$ is undefined (where [gamma] returns `NaN` or `Inf`). This is $x=0.0, -1, -2, \ldots$.
    ///
    /// [gamma]: crate::special::Gamma::gamma()
    fn gammasgn(self) -> Self;

}

macro_rules! float_gamma_impl {
    ($($T: ty)*) => ($(
        impl Gamma for $T {
            //#[inline(always)]
            //fn gamma(self) -> Self {
                //r_gamma(self)
            //}

            //#[inline(always)]
            //fn lgamma(self) -> Self {
                //r_lgamma(self)
            //}

            //#[inline(always)]
            //fn lngamma(self) -> Self {
                //if self < 0.0 {
                    //return Self::NAN;
                //}
                //self.lgamma()
            //}

            //#[inline(always)]
            //fn rgamma(self) -> Self {
                //r_rgamma(self)
            //}
        }

        impl RealGamma for $T {
            #[inline(always)]
            fn gammasgn(self) -> Self {
                r_gammasgn(self)
            }

            //#[inline(always)]
            //fn poch(self, m: Self) -> Self {
                //r_poch(self, m)
            //}

            //#[inline(always)]
            //fn gammainc(self, s: Self) -> Self {
                //r_gammainc(self, s)
            //}
        }
    )*)
}

float_gamma_impl! {f32 f64}
