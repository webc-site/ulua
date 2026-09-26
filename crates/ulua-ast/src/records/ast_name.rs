use alloc::string::String;
use core::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result},
  ptr::null,
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_common::records::dense_hash_table::DenseDefault;

/// Port of `Luau::AstName` (Ast/include/Luau/Ast.h:21-60).
///
/// `value` is an interned C string owned by `AstNameTable`; equality/hash are
/// pointer identity (reference `operator==(const AstName&)`), while ordering is
/// content-based `strcmp` with a pointer fallback when either side is null
/// (reference `operator<`). The two are consistent under the interning
/// invariant: equal content from the same table implies the same pointer.
///
/// 存储面批 2 改型：cpp `const char* value`（Ast.h:23）→ `*const u8`。同一字节
/// 域（char 与 u8 逐位同宽），NUL 结尾/终止契约零增删，指针 identity 判等与
/// 内容序均不受符号性影响。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstName {
  pub value: *const u8,
  pub len: u32,
}

impl PartialOrd for AstName {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AstName {
  fn cmp(&self, other: &Self) -> Ordering {
    if !self.is_null() && !other.is_null() {
      match self.as_bytes().cmp(other.as_bytes()) {
        Ordering::Equal => self.value.cmp(&other.value),
        o => o,
      }
    } else {
      self.value.cmp(&other.value)
    }
  }
}

impl AstName {
  /// 唯一构造空名
  #[inline]
  pub const fn new() -> Self {
    Self {
      value: null(),
      len: 0,
    }
  }

  /// Returns the name as a byte slice in O(1) without any strlen scanning.
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    if self.value.is_null() {
      &[]
    } else {
      unsafe { from_raw_parts(self.value, self.len as usize) }
    }
  }

  /// Tries to return the name as a UTF-8 `&str`, or None if null or invalid UTF-8.
  #[inline]
  pub fn as_str(&self) -> Option<&str> {
    from_utf8(self.as_bytes()).ok()
  }

  /// Returns the name as a `&str`, or `""` if null or invalid UTF-8.
  #[inline]
  pub fn as_str_or_empty(&self) -> &str {
    self.as_str().unwrap_or("")
  }

  /// 从纯 Rust 静态切片构造非驻留名，不再需要任何 C 风格 \0 结尾
  #[inline]
  pub const fn from_static(name: &'static [u8]) -> Self {
    Self {
      value: name.as_ptr(),
      len: name.len() as u32,
    }
  }

  /// 从纯 Rust 静态字符串构造
  #[inline]
  pub const fn from_str(name: &'static str) -> Self {
    Self::from_static(name.as_bytes())
  }

  /// 从原始指针与长度构造
  #[inline]
  pub const fn from_raw_parts(value: *const u8, len: u32) -> Self {
    Self { value, len }
  }

  /// Returns the length of the string in bytes.
  #[inline]
  pub fn len(&self) -> usize {
    self.len as usize
  }

  /// Returns true if the string is empty or null.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns whether this name is null.
  #[inline]
  pub fn is_null(&self) -> bool {
    self.value.is_null()
  }
}

// 作为 `DenseHashMap`/`DenseHashSet` 键时的空槽占位值：null 指针哨兵，与各调用点
// 旧形 `new(AstName::new())`/`new(AstName::default())` 逐位等价（本类型 `Default`
// 即 `Self::new()`）。契约（见 ulua-common `dense_hash_table` 模块文档）：占用与否
// 由位图判定、哨兵可存取；真实键由 `AstNameTable` 驻留产生、指针恒非 null，故 null
// 哨兵不与任何真实键撞车，`DenseHashMap::<AstName, _>::default()` 门面可安全使用。
impl DenseDefault for AstName {
  fn dense_default() -> Self {
    Self::new()
  }
}

impl PartialEq<&str> for AstName {
  #[inline]
  fn eq(&self, other: &&str) -> bool {
    !self.is_null() && self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<str> for AstName {
  #[inline]
  fn eq(&self, other: &str) -> bool {
    !self.is_null() && self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<AstName> for &str {
  #[inline]
  fn eq(&self, other: &AstName) -> bool {
    other == self
  }
}

impl PartialEq<&[u8]> for AstName {
  #[inline]
  fn eq(&self, other: &&[u8]) -> bool {
    !self.is_null() && self.as_bytes() == *other
  }
}

impl Display for AstName {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.as_str() {
      Some(s) => f.write_str(s),
      None => {
        let lossy = String::from_utf8_lossy(self.as_bytes());
        f.write_str(&lossy)
      }
    }
  }
}
