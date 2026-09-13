#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DebugLocal<'a> {
  pub(crate) varname: &'a str,
  pub(crate) reg: u8,
  pub(crate) startpc: u32,
  pub(crate) endpc: u32,
}
