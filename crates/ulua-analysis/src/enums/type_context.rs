#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum TypeContext {
  /// the default context
  #[default]
  Default,
  /// inside of a condition
  Condition,
}
