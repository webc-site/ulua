use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DeprecatedApiUsed {
  pub symbol: String,
  pub use_instead: String,
}
