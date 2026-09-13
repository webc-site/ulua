pub use crate::records::alias_options::AliasOptions;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ConfigOptions {
  pub compat: bool,
  pub alias_options: Option<AliasOptions>,
}
