use crate::enums::variance::Variance;

#[derive(Debug, Clone)]
pub struct Resetter {
  pub old_value: Variance,
  pub variance: *mut Variance,
}
