use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DebugLocal {
  pub(crate) varname: String,
  pub(crate) reg: u8,
  pub(crate) startpc: u32,
  pub(crate) endpc: u32,
}
