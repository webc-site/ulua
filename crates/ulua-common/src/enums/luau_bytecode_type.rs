//! Source: `Common/include/Luau/Bytecode.h`

use crate::records::dense_hash_table::DenseDefault;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuauBytecodeType(pub u16);

impl DenseDefault for LuauBytecodeType {
  fn dense_default() -> Self {
    Self::LBC_TYPE_NIL
  }
}

impl LuauBytecodeType {
  pub const LBC_TYPE_NIL: Self = Self(0);
  pub const LBC_TYPE_BOOLEAN: Self = Self(1);
  pub const LBC_TYPE_NUMBER: Self = Self(2);
  pub const LBC_TYPE_STRING: Self = Self(3);
  pub const LBC_TYPE_TABLE: Self = Self(4);
  pub const LBC_TYPE_FUNCTION: Self = Self(5);
  pub const LBC_TYPE_THREAD: Self = Self(6);
  pub const LBC_TYPE_USERDATA: Self = Self(7);
  pub const LBC_TYPE_VECTOR: Self = Self(8);
  pub const LBC_TYPE_BUFFER: Self = Self(9);
  pub const LBC_TYPE_INTEGER: Self = Self(10);

  pub const LBC_TYPE_ANY: Self = Self(15);

  pub const LBC_TYPE_TAGGED_USERDATA_BASE: Self = Self(64);
  pub const LBC_TYPE_TAGGED_USERDATA_END: Self = Self(64 + 32);

  pub const LBC_TYPE_OPTIONAL_BIT: Self = Self(1 << 7);

  pub const LBC_TYPE_INVALID: Self = Self(256);

  /// 基础类型名（dump 展示用单一真相）：此前 ulua-bytecode / ulua-code-gen 各持一份
  /// 镜像 match 表，曾因手抄判别式漂移出 bug（INTEGER=3 平移整表）。tagged-userdata
  /// 区间（64..96）不属于基础类型，由调用方自行处理。
  pub const fn base_name(self) -> Option<&'static str> {
    match self {
      Self::LBC_TYPE_NIL => Some("nil"),
      Self::LBC_TYPE_BOOLEAN => Some("boolean"),
      Self::LBC_TYPE_NUMBER => Some("number"),
      Self::LBC_TYPE_STRING => Some("string"),
      Self::LBC_TYPE_TABLE => Some("table"),
      Self::LBC_TYPE_FUNCTION => Some("function"),
      Self::LBC_TYPE_THREAD => Some("thread"),
      Self::LBC_TYPE_USERDATA => Some("userdata"),
      Self::LBC_TYPE_VECTOR => Some("vector"),
      Self::LBC_TYPE_BUFFER => Some("buffer"),
      Self::LBC_TYPE_INTEGER => Some("integer"),
      Self::LBC_TYPE_ANY => Some("any"),
      _ => None,
    }
  }
}
