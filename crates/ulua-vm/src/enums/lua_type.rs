use core::ffi::c_int;

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

impl LuaType {
  /// `lua_type` 等 API 返回的 `c_int` → 枚举。
  ///
  /// 判别式一律取枚举自身（不另写字面量），故本文件仍是类型的唯一真相；
  /// 未知 tag 返回 `Err`，由调用方决定兜底语义。
  pub const fn from_c_int(tt: c_int) -> Option<Self> {
    match tt {
      v if v == Self::None as c_int => Some(Self::None),
      v if v == Self::Nil as c_int => Some(Self::Nil),
      v if v == Self::Boolean as c_int => Some(Self::Boolean),
      v if v == Self::LightUserData as c_int => Some(Self::LightUserData),
      v if v == Self::Number as c_int => Some(Self::Number),
      v if v == Self::Integer as c_int => Some(Self::Integer),
      v if v == Self::Vector as c_int => Some(Self::Vector),
      v if v == Self::String as c_int => Some(Self::String),
      v if v == Self::Table as c_int => Some(Self::Table),
      v if v == Self::Function as c_int => Some(Self::Function),
      v if v == Self::UserData as c_int => Some(Self::UserData),
      v if v == Self::Thread as c_int => Some(Self::Thread),
      v if v == Self::Buffer as c_int => Some(Self::Buffer),
      v if v == Self::Class as c_int => Some(Self::Class),
      v if v == Self::Object as c_int => Some(Self::Object),
      v if v == Self::DeadKey as c_int => Some(Self::DeadKey),
      v if v == Self::Proto as c_int => Some(Self::Proto),
      v if v == Self::Upval as c_int => Some(Self::Upval),
      _ => None,
    }
  }
}
