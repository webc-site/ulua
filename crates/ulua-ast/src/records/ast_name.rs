use alloc::string::String;
use core::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result},
  hash::{Hash, Hasher},
  ptr::{NonNull, null},
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_common::records::dense_hash_table::DenseDefault;

/// Port of `Luau::AstName` (Ast/include/Luau/Ast.h:21-60).
///
/// 身份值化（review.md §2）：cpp `const char* value`（Ast.h:23）→ 私有
/// `ptr: Option<NonNull<u8>>`。指向名表驻留串的指针仍是**唯一**身份来源，只是
/// 缺席由类型层的 `None` 表达、不再有 `null()` 哨兵；`Option<NonNull>` 走 niche
/// 优化，`Option`/`size`/对齐与原 `(指针, len)` 逐位同形。派生 `PartialEq`/`Eq`
/// 即「指针地址 + len」的逐字段相等判定（cpp `operator==(const AstName&)`），与原
/// 裸指针字段版派生实现逐位等价；`Hash` 手写为同一字节流（见下），使
/// `DenseHashMap`/`DenseHashSet` 的桶位与旧实现完全一致——`Option` 的派生实现会
/// 先写入判别字节，改的是哈希值而非判等语义，故这里逐位守住。
///
/// 指针不变式：`ptr` 恒出自 `AstNameTable` 驻留缓冲（NUL 结尾、比名字值长寿）
/// 或调用方证明存活的切片（[`AstName::from_static`]、
/// [`AstName::from_raw_parts`]）；「同表同内容 ⇔ 同指针」的驻留不变式使指针判等
/// 与内容序一致。解引用只发生在 [`AstName::as_bytes`] 一处（arena 边界门面）。
///
/// 序为内容比较、指针打平，且保留「任一侧为缺席名 → 只比指针」的特殊形态
/// （`None` 的 Ord 位次恒先于任意驻留指针，与旧 `nullptr` 地址 0 的指针比较
/// 同序）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstName {
  /// 名表驻留串的指针身份；`None` 即 cpp 的 `nullptr`「无名」态。
  ptr: Option<NonNull<u8>>,
  pub len: u32,
}

impl PartialOrd for AstName {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AstName {
  fn cmp(&self, other: &Self) -> Ordering {
    // 缺席名（None）只比指针位序；两名俱在时内容优先、指针打平（cpp `operator<`）。
    match (self.ptr, other.ptr) {
      (Some(a), Some(b)) => match self.as_bytes().cmp(other.as_bytes()) {
        Ordering::Equal => a.cmp(&b),
        o => o,
      },
      _ => self.ptr.cmp(&other.ptr),
    }
  }
}

impl AstName {
  /// 唯一构造空名（缺席态）
  #[inline]
  pub const fn new() -> Self {
    Self { ptr: None, len: 0 }
  }

  /// Returns the name as a byte slice in O(1) without any strlen scanning.
  /// 缺席名折叠为空切片（cpp 判 `nullptr` 后同形）。
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.ptr.map_or(&[], |p| {
      // Safety: `ptr` 由类型不变式保证出自驻留缓冲或调用方证明存活的切片，
      // 与 `len` 成对写入、区间 `[p, p + len)` 全程可读；u8 与 cpp `char`
      // 同字节域。
      unsafe { from_raw_parts(p.as_ptr().cast_const(), self.len as usize) }
    })
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
      ptr: NonNull::new(name.as_ptr().cast_mut()),
      len: name.len() as u32,
    }
  }

  /// 从纯 Rust 静态字符串构造
  #[inline]
  pub const fn from_str(name: &'static str) -> Self {
    Self::from_static(name.as_bytes())
  }

  /// 从原始指针与长度构造（arena/名表驻留边界：指针身份原样透传，null 折为
  /// 缺席态，不构造引用、不解引用）。
  #[inline]
  pub const fn from_raw_parts(value: *const u8, len: u32) -> Self {
    Self {
      ptr: NonNull::new(value.cast_mut()),
      len,
    }
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

  /// Returns whether this name is null（cpp `value == nullptr` 的类型化形态）。
  #[inline]
  pub fn is_null(&self) -> bool {
    self.ptr.is_none()
  }

  /// 指针身份桥（与 `Handle::as_ptr` / [`crate::records::node_handle::Node::as_ptr`]
  /// 同型先例）：`None` 折回 `null`，仅供 extern "C-unwind" 回调与尚未句柄化的
  /// 裸指针消费点透传地址值，不构成解引用许可。
  #[inline]
  pub fn as_ptr(&self) -> *const u8 {
    self.ptr.map_or(null(), |p| p.as_ptr().cast_const())
  }

  /// [`LexemeData`](crate::records::lexeme::LexemeData) 的指针臂与 `ptr` 同型，
  /// 词法路径零往返互转（crate 内部，不扩公共 API 面）。
  #[inline]
  pub(crate) const fn as_opt_ptr(&self) -> Option<NonNull<u8>> {
    self.ptr
  }

  /// [`AstName::as_opt_ptr`] 的构造向配对；`len` 原样携带（含缺席名伴生长度的
  /// 历史形态，保证与旧字段直写逐位等价）。
  #[inline]
  pub(crate) const fn from_opt_ptr(ptr: Option<NonNull<u8>>, len: u32) -> Self {
    Self { ptr, len }
  }
}

// 哈希字节流与旧裸指针字段版派生实现逐位一致：先指针地址、后 `len`
// （`None` 折回地址 0，正是旧 `null()` 字段值）。`Option<NonNull>` 的派生实现会
// 额外写入判别字节，不损判等语义但会挪动 `DenseHashMap`/`DenseHashSet` 桶位，
// 故手写守住。
impl Hash for AstName {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_ptr().hash(state);
    self.len.hash(state);
  }
}

// 作为 `DenseHashMap`/`DenseHashSet` 键时的空槽占位值：缺席名（`ptr: None`）
// 哨兵，与各调用点旧形 `new(AstName::new())`/`new(AstName::default())` 逐位等价
// （本类型 `Default` 即 `Self::new()`）。契约（见 ulua-common `dense_hash_table`
// 模块文档）：占用与否由位图判定、哨兵可存取；真实键由 `AstNameTable` 驻留产生、
// 指针恒非缺席，故哨兵不与任何真实键撞车，`DenseHashMap::<AstName, _>::default()`
// 门面可安全使用。
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
