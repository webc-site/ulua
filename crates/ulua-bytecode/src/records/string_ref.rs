use core::{
  ffi::c_char,
  fmt::{self, Display, Formatter},
  hash::{Hash, Hasher},
  marker::PhantomData,
  ptr, slice,
  str::{Utf8Error, from_utf8},
};
use std::{borrow::Cow, string::String};

/// cpp `BytecodeBuilder::StringRef` —— 指向外部所有字节的**非拥有**视图。
///
/// 寿命参数 `'a` 就是那段字节的存活期：`PhantomData<&'a [u8]>` 让借用检查真正
/// 约束住它。没有这个参数时（旧实现），`from_slice(&'temp [u8])` 会把 `'temp`
/// 抹成"任意"，于是 safe API 能铸出一个可以在源缓冲析构后继续读的视图
/// （局部 `Vec` 先 drop 即 UAF，`as_bytes`/`memchr` 越界读）。
#[derive(Debug, Clone, Copy, Default)]
pub struct StringRef<'a> {
  pub(crate) data: *const u8,
  pub(crate) length: usize,
  /// 唯一用途：把 `data`/`length` 与 `'a` 关联起来，使本结构在 `'a` 结束前不可使用。
  _marker: PhantomData<&'a [u8]>,
}

impl<'a> StringRef<'a> {
  /// cpp `StringRef(const char* data, size_t length)`。
  ///
  /// 裸指针没有可验证的寿命，这里只能按 cpp 的约定把它当成 `'static`：
  /// 传入的缓冲区必须在本 `StringRef` 及其所有副本的整个使用期内保持有效
  /// （cpp 侧即"字符串由 Allocator/编译期常量池持有"）。
  /// 有 `&[u8]` 可用时请优先走 [`Self::from_slice`]，那条路径寿命是被检查的。
  pub fn new(data: *const c_char, length: usize) -> Self {
    Self {
      data: data as *const u8,
      length,
      _marker: PhantomData,
    }
  }

  /// `DenseHashMap` 空键哨兵（cpp `StringRef{nullptr, 0}`）。
  ///
  /// 与内容型 [`PartialEq`] 配合：`{null, 0}` 只与另一个 `{null, 0}` 相等，
  /// 而任何非空切片（哪怕长度为 0）都走内容比较，因此哨兵不可达。
  pub const NULL: Self = Self {
    data: ptr::null(),
    length: 0,
    _marker: PhantomData,
  };

  /// Construct a `StringRef` from a byte slice.
  #[inline]
  pub fn from_slice(bytes: &'a [u8]) -> Self {
    Self {
      data: bytes.as_ptr(),
      length: bytes.len(),
      _marker: PhantomData,
    }
  }

  /// 把一段生命周期受限的字节视图升格为 `StringRef<'static>`。
  ///
  /// 存在的理由：`BytecodeBuilder` 的 `string_table` / `debug_strings` 按 cpp
  /// 语义只借用调用方的名字到 `end_function` 把字节码写出去为止，而 `BytecodeBuilder`
  /// 本身不带寿命参数（cpp 里这就是个自引用对象）。凡是要把受限借用塞进这些容器的
  /// 地方，都得在这里显式承认这条契约，而不是让 `from_slice` 静默放过。
  ///
  /// # Safety
  /// 调用方必须保证：从本次调用起到持有者（builder）不再读该视图为止
  /// （即 `end_function` / `finalize` 把它序列化出去），`bytes` 指向的缓冲
  /// 不被释放、不被移动（`String`/`Vec` 扩容会移动缓冲）。
  pub const unsafe fn from_slice_static(bytes: &[u8]) -> StringRef<'static> {
    StringRef {
      data: bytes.as_ptr(),
      length: bytes.len(),
      _marker: PhantomData,
    }
  }

  /// Returns the underlying byte slice.
  #[inline]
  pub fn as_bytes(&self) -> &'a [u8] {
    if self.data.is_null() {
      &[]
    } else {
      // SAFETY：`StringRef<'a>` 的不变量是 `data` 指向 `'a` 内存活的 `length` 字节。
      unsafe { slice::from_raw_parts(self.data, self.length) }
    }
  }

  /// Tries to convert the string ref into a UTF-8 `&str`.
  #[inline]
  pub fn as_str(&self) -> Result<&'a str, Utf8Error> {
    from_utf8(self.as_bytes())
  }

  /// Converts the string ref to a Cow string, replacing invalid UTF-8 sequences.
  #[inline]
  pub fn to_string_lossy(&self) -> Cow<'a, str> {
    String::from_utf8_lossy(self.as_bytes())
  }

  /// Returns the length in bytes.
  #[inline]
  pub fn len(&self) -> usize {
    self.length
  }

  /// Returns true if the string has length 0.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  /// Returns true if the pointer is null.
  #[inline]
  pub fn is_null(&self) -> bool {
    self.data.is_null()
  }
}

impl<'a> From<&'a [u8]> for StringRef<'a> {
  #[inline]
  fn from(bytes: &'a [u8]) -> Self {
    Self::from_slice(bytes)
  }
}

impl<'a> From<&'a str> for StringRef<'a> {
  #[inline]
  fn from(s: &'a str) -> Self {
    Self::from_slice(s.as_bytes())
  }
}

impl Display for StringRef<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.as_str() {
      Ok(s) => f.write_str(s),
      Err(_) => f.write_str(&self.to_string_lossy()),
    }
  }
}

// C++ `BytecodeBuilder::StringRef::operator==`:
//   (data && other.data) ? (length == other.length && memcmp(...) == 0) : (data == other.data)
// i.e. content comparison when both pointers are non-null, otherwise raw-pointer
// equality (so the DenseHashMap's {null, 0} empty-key sentinel still matches itself).
// The derived impl compared the raw pointer, so the same string reached via two
// different buffers (e.g. an AstName key vs a string-literal key, both "f") failed to
// dedup and produced duplicate string-table / constant entries.
//
// 两侧寿命参数相互独立：判等只读字节内容，不跨寿命持有，故无需 `'a == 'b`。
impl PartialEq for StringRef<'_> {
  fn eq(&self, other: &Self) -> bool {
    if !self.data.is_null() && !other.data.is_null() {
      self.length == other.length
        && (self.data == other.data || self.as_bytes() == other.as_bytes())
    } else {
      self.data == other.data
    }
  }
}

impl PartialEq<str> for StringRef<'_> {
  #[inline]
  fn eq(&self, other: &str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl<'b> PartialEq<&'b str> for StringRef<'_> {
  #[inline]
  fn eq(&self, other: &&'b str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<StringRef<'_>> for &str {
  #[inline]
  fn eq(&self, other: &StringRef<'_>) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl Eq for StringRef<'_> {}

// C++ `StringRefHash` hashes the content range (`hashRange(data, length)`); equal
// content must hash equally for the dedup map.
impl Hash for StringRef<'_> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_bytes().hash(state);
  }
}
