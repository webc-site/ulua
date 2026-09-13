#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum LuaStatus {
  #[default]
  Ok = 0,
  Yield = 1,
  ErrRun = 2,
  ErrSyntax = 3,
  ErrMem = 4,
  ErrErr = 5,
  Break = 6,
}
