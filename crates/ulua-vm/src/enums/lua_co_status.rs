#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LuaCoStatus {
  CoRun = 0,
  CoSus = 1,
  CoNor = 2,
  CoFin = 3,
  CoErr = 4,
}
