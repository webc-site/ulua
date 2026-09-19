use core::ffi::c_char;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::type_constant_folding::Type;

/// 字符串常量存储：指向借用数据的裸指针 + 字节长度（与 C++ `Constant` 一致，不拷贝）
#[derive(Clone, Copy, Debug)]
pub struct ConstantStr {
  pub(crate) ptr: *const c_char,
  pub(crate) len: u32,
}

/// 以带载荷 enum 取代 C++ 的 `type` 标签 + union，类型与数据合一，消除 union 误读的 UB 风险。
/// 尺寸与原 `#[repr(C)]` 结构相同（24 字节），热路径按值拷贝无回退。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Constant {
  /// 未知 / 不可折叠
  Unknown,
  Nil,
  Boolean(bool),
  Number(f64),
  Integer(i64),
  Vector([f32; 4]),
  /// 表常量，载荷为 `constant_tables` 的下标
  Table(usize),
  Str(ConstantStr),
}

impl Constant {
  /// 由裸指针 + 长度构造字符串常量（fold 拼接与 C API 回填共用）
  #[inline]
  pub(crate) fn string(ptr: *const c_char, len: u32) -> Self {
    Self::Str(ConstantStr { ptr, len })
  }

  /// 类型标签，供纯标签判断使用（不读数据，无 unsafe）
  #[inline]
  pub fn r#type(&self) -> Type {
    match self {
      Self::Unknown => Type::Unknown,
      Self::Nil => Type::Nil,
      Self::Boolean(_) => Type::Boolean,
      Self::Number(_) => Type::Number,
      Self::Integer(_) => Type::Integer,
      Self::Vector(_) => Type::Vector,
      Self::Table(_) => Type::Table,
      Self::Str(_) => Type::String,
    }
  }

  #[inline]
  pub fn is_unknown(&self) -> bool {
    matches!(self, Self::Unknown)
  }

  /// 字符串字节长度；非字符串常量恒为 0（与原 `string_length` 字段语义一致）
  #[inline]
  pub(crate) fn string_len(&self) -> u32 {
    match self {
      Self::Str(s) => s.len,
      _ => 0,
    }
  }
}

impl DenseDefault for Constant {
  fn dense_default() -> Self {
    Self::Unknown
  }
}

impl Default for Constant {
  fn default() -> Self {
    Self::Unknown
  }
}
