use std::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct DebugLocal {
  pub(crate) varname: String,
  pub(crate) reg: u8,
  pub(crate) startpc: u32,
  pub(crate) endpc: u32,
}
