use core::fmt::{Display, Formatter, Result};
use std::error::Error;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NormalizerHitLimits;

impl Display for NormalizerHitLimits {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Normalizer hit limits")
  }
}

#[cfg(feature = "std")]
impl Error for NormalizerHitLimits {}

unsafe impl Send for NormalizerHitLimits {}
unsafe impl Sync for NormalizerHitLimits {}
