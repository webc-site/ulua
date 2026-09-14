#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LuaType {
  None = -1,
  Nil = 0,
  Boolean = 1,

  LightUserData = 2,
  Number = 3,
  Integer = 4,
  Vector = 5,

  String = 6,

  Table = 7,
  Function = 8,
  UserData = 9,
  Thread = 10,
  Buffer = 11,
  Class = 12,
  Object = 13,

  DeadKey = 14,

  Proto = 15,
  Upval = 16,
}

pub const LUA_T_COUNT: LuaType = LuaType::DeadKey;
pub const LUA_TNONE: i32 = LuaType::None as i32;
