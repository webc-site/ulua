use core::ptr::null;

use ulua_ast::records::ast_array::AstArray;
use ulua_common::{
  functions::c_slice::c_slice, macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_table::DenseDefault,
};

/// 字符串常量存储：指向借用数据的裸指针 + 字节长度（与 C++ `Constant` 一致，不拷贝）
#[derive(Clone, Copy, Debug)]
pub struct ConstantStr {
  pub(crate) ptr: *const u8,
  pub len: u32,
}

impl ConstantStr {
  /// 借用为字节切片；空串（null 指针或零长）返回空切片
  pub(crate) fn bytes(&self) -> &[u8] {
    // Safety: ptr/len 由调用方保证构成合法字节区域（或 null 配 0）。
    unsafe { c_slice(self.ptr, self.len as usize) }
  }
}

/// 以带载荷 enum 取代 C++ 的 `type` 标签 + union，类型与数据合一，消除 union 误读的 UB 风险。
/// 尺寸与原 `#[repr(C)]` 结构相同（24 字节），热路径按值拷贝无回退。
/// 不派生 `PartialEq`：字符串相等须比较内容而非指针，统一走 `constants_equal`。
#[derive(Clone, Copy, Debug, Default)]
pub enum Constant {
  /// 未知 / 不可折叠
  #[default]
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
  #[inline]
  pub const fn boolean(v: bool) -> Self {
    Self::Boolean(v)
  }

  #[inline]
  pub const fn number(v: f64) -> Self {
    Self::Number(v)
  }

  #[inline]
  pub const fn integer(v: i64) -> Self {
    Self::Integer(v)
  }

  #[inline]
  pub const fn vector(v: [f32; 4]) -> Self {
    Self::Vector(v)
  }

  #[inline]
  pub const fn nil() -> Self {
    Self::Nil
  }

  /// 由裸指针 + 长度构造字符串常量（fold 拼接与 C API 回填共用）
  #[inline]
  pub(crate) fn string(ptr: *const u8, len: u32) -> Self {
    Self::Str(ConstantStr { ptr, len })
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

  /// 批 2 存储面：cpp `Constant` 字符串以 `char*` 存、`AstArray<char>` 外露，
  /// Rust 侧字节域收为 `AstArray<u8>`（`ConstantStr.ptr` 为原生 `*const u8`），
  /// 下游 sref/字面量视图按 u8 切片消费。
  pub fn get_string(&self) -> AstArray<u8> {
    let ConstantStr { ptr, len } = self.as_str();
    // 指针借用自 AST/字符串表，沿用 C++ 的 AstArray 可变指针表示
    AstArray {
      data: ptr.cast_mut(),
      size: len as usize,
    }
  }

  /// 以字节切片形式返回字符串常量。
  ///
  /// data 指针存活于常量自身的存储（比 `self` 更久），故切片的生命周期
  /// 绑定到 `&self` 而非临时的 `AstArray`。
  #[inline]
  pub fn get_string_bytes(&self) -> &[u8] {
    match self {
      Constant::Str(s) => s.bytes(),
      // 契约违例：非字符串常量取字节
      _ => {
        LUAU_ASSERT!(false);
        &[]
      }
    }
  }

  /// 取出字符串载荷；非字符串常量属契约违例，断言拦截
  #[inline]
  pub(crate) fn as_str(&self) -> ConstantStr {
    LUAU_ASSERT!(matches!(self, Constant::Str(_)));
    match self {
      Constant::Str(s) => *s,
      _ => ConstantStr {
        ptr: null(),
        len: 0,
      },
    }
  }

  pub(crate) fn is_truthful(&self) -> bool {
    LUAU_ASSERT!(!self.is_unknown());
    match self {
      Self::Nil => false,
      Self::Boolean(b) => *b,
      // nil、false 之外皆真；Unknown 由前置断言排除
      _ => true,
    }
  }
}

impl DenseDefault for Constant {
  fn dense_default() -> Self {
    Self::Unknown
  }
}
