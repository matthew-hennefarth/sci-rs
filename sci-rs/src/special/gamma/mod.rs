
//! Various functions related to the Gamma function.

mod gamma_trait;
pub use gamma_trait::*;

pub(crate) mod gamma_util;

mod r_gammasgn;

mod real_gamma_impl {
    pub(crate) use super::r_gammasgn::*;
}
