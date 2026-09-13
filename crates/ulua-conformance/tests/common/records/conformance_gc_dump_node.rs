#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceGcDumpNode {
  pub ptr: usize,
  pub tag: u8,
  pub memcat: u8,
  pub size: usize,
  pub name: String,
}
