#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum ConfigStatus {
  Absent,
  Ambiguous,
  PresentJson,
  PresentLuau,
}
