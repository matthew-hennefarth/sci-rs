
use crate::special::gamma::real_gamma_impl::*; 

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
