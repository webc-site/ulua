#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NormalizationResult {
  // The operation returned true or succeeded.
  True,
  // The operation returned false or failed.
  False,
  // Resource limits were hit, invalidating all normalized types.
  HitLimits,
}
