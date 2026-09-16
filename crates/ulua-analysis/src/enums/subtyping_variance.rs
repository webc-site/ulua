#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum SubtypingVariance {
  #[default]
  Invalid,
  Covariant,
  Contravariant,
  Invariant,
}
