#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}
