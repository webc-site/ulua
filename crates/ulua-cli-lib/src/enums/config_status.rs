#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigStatus {
  Absent,
  Ambiguous,
  PresentJson,
  PresentLuau,
}
