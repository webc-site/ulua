#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BuiltinImplType {
  #[default]
  None,
  UsesFallback,
  Full,
}
