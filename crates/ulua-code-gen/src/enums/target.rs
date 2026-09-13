#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Target {
  #[default]
  Host,
  A64,
  A64NoFeatures,
  X64Windows,
  X64SystemV,
}
