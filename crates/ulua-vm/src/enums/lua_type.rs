#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
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
  /// cpp lua.h:114 `LUA_T_ALL`：全部 Luau 类型计数哨兵（Upval+1），
  /// 供 onallocate 等回调标识“非真实类型 tag”
  pub const ALL_SENTINEL: i32 = Self::Upval as i32 + 1;

  /// `lua_type` 等 API 返回的 `i32` → 枚举。
  #[inline(always)]
  pub const fn from_c_int(tt: i32) -> Option<Self> {
    match tt {
      -1 => Some(Self::None),
      0 => Some(Self::Nil),
      1 => Some(Self::Boolean),
      2 => Some(Self::LightUserData),
      3 => Some(Self::Number),
      4 => Some(Self::Integer),
      5 => Some(Self::Vector),
      6 => Some(Self::String),
      7 => Some(Self::Table),
      8 => Some(Self::Function),
      9 => Some(Self::UserData),
      10 => Some(Self::Thread),
      11 => Some(Self::Buffer),
      12 => Some(Self::Class),
      13 => Some(Self::Object),
      14 => Some(Self::DeadKey),
      15 => Some(Self::Proto),
      16 => Some(Self::Upval),
      _ => None,
    }
  }

  /// tag 展示名（"tnil"/"tboolean"...），对应 cpp IrDump 的 tag 名表；
  /// 此前 ulua-code-gen 手抄了一份 17 常量镜像表，收敛至此单一真相。
  #[inline]
  pub const fn tag_name(self) -> &'static str {
    match self {
      Self::None => unreachable!(),
      Self::Nil => "tnil",
      Self::Boolean => "tboolean",
      Self::LightUserData => "tlightuserdata",
      Self::Number => "tnumber",
      Self::Integer => "tinteger",
      Self::Vector => "tvector",
      Self::String => "tstring",
      Self::Table => "ttable",
      Self::Function => "tfunction",
      Self::UserData => "tuserdata",
      Self::Thread => "tthread",
      Self::Buffer => "tbuffer",
      Self::Class => "tclass",
      Self::Object => "tobject",
      Self::DeadKey => "tdeadkey",
      Self::Proto => "tproto",
      Self::Upval => "tupval",
    }
  }
}
