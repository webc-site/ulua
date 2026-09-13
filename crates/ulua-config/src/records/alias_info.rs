use alloc::string::String;

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct AliasInfo {
  pub value: String,
  pub config_location: String,
  pub original_case: String,
}
