use alloc::string::String;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AliasOptions {
  pub config_location: Option<String>,
  pub overwrite_aliases: bool,
}
