use core::{
  error::Error,
  fmt::{Display, Formatter, Result},
};
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NormalizerHitLimits;

impl Display for NormalizerHitLimits {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Normalizer hit limits")
  }
}

impl Error for NormalizerHitLimits {}
