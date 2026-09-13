#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Status {
  Cached,
  ModuleRead,
  ErrorReported,
}
