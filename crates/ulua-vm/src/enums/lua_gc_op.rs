#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
#[repr(i32)]
pub enum LuaGcOp {
  Stop = 0,
  Restart = 1,
  Collect = 2,
  Count = 3,
  Countb = 4,
  Isrunning = 5,
  Step = 6,
  Setgoal = 7,
  Setstepmul = 8,
  Setstepsize = 9,
  /// cpp LUA_GCISPAUSED（lua.h:333-334）：GC 是否停在 GCSpause 态
  IsPaused = 10,
}
