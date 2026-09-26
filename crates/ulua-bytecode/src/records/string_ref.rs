use core::{
  fmt::{self, Display, Formatter},
  hash::{Hash, Hasher},
  slice::from_raw_parts,
  str::{Utf8Error, from_utf8},
};
use std::{borrow::Cow, string::String};

/// cpp `BytecodeBuilder::StringRef`（`Bytecode/include/Luau/BytecodeBuilder.h:40`）。
///
/// 上游是 `{const char* data; size_t length;}` 的视图二元组；这里用 `Option<&'a [u8]>`
/// 表达同一语义，并且：
/// - 借用取代裸指针，视图的可读区间由生命周期 `'a` 静态保证（纯 safe 代码无法造出悬垂读）；
/// - 内部是 `Cow`：编译器路径（`sref()`/AST 名表）继续零拷贝借用（`Borrowed`）；
///   图序列化路径（`toFunctionBytecode`）的字符串数据在 `BcFunction` 内、且构建期间仍被
///   `&mut` 改写，借用模型无法表达（上游靠 `string_view` 别名，见 `BytecodeGraph.cpp:322-398`），
///   这些入口用 `StringRef::owned` 拷贝一份（`Owned`），代价仅限 debug 名与常量串；
/// - `None` 精确对应上游 `data == nullptr`，用于 `stringTable` 这个 `DenseHashMap` 的
///   空槽哨兵；`Some(&[])` 对应上游 `StringRef{"", 0}`，是一个**合法**的空串键
///   （cpp 构造函数里 `LUAU_ASSERT(stringTable.find(StringRef{"", 0}) == nullptr)` 断言的
///   正是这两者不混淆）。
#[derive(Debug, Clone)]
pub struct StringRef<'a> {
  bytes: Option<Cow<'a, [u8]>>,
}

/// cpp `StringRef{}` 默认构造：`data = nullptr, length = 0`。
impl Default for StringRef<'_> {
  #[inline]
  fn default() -> Self {
    Self { bytes: None }
  }
}

impl<'a> StringRef<'a> {
  /// cpp `StringRef(const char* data, size_t length)`：把外部（AST 名表等 arena）持有的
  /// 字节串包成视图。仓库里唯一的入口是 `Compiler` 的 `sref()`，其正确性依赖
  /// 「名表寿命覆盖字节码构建过程」这一调用方约定，因此这里必须显式 `unsafe`。
  /// 收 `*const u8`（cpp char 与 u8 同一字节域）：本类型不是 C ABI 面，
  /// `*const c_char` 转换留在 compiler 的 FFI 边界。
  ///
  /// # Safety
  /// - `data..data.add(length)` 在 `'a` 全程可读（或 `length == 0` 时允许 `data` 悬垂但非空）；
  /// - `data` 为 null 时本函数归一为哨兵（`default()`）并忽略 `length`：null 是
  ///   `DenseHashMap` 的空槽哨兵状态，带长度的空指针会被当成合法键从而破坏哨兵不变量。
  pub unsafe fn new(data: *const u8, length: usize) -> Self {
    if data.is_null() {
      Self::default()
    } else {
      // Safety: 由调用方保证 `data` 在 `'a` 内可读 `length` 字节
      Self {
        bytes: Some(Cow::Borrowed(unsafe { from_raw_parts(data, length) })),
      }
    }
  }

  /// 由字节切片构造（对应 cpp `StringRef{bytes.data(), bytes.len()}`）。
  #[inline]
  pub fn from_slice(bytes: &'a [u8]) -> Self {
    Self {
      bytes: Some(Cow::Borrowed(bytes)),
    }
  }

  /// 拥有所有权的字符串（`Cow::Owned`）。供数据源本身在被序列化对象内、无法按 `'a`
  /// 借用给出的路径使用（`toFunctionBytecode` 的 debug 名与常量串）；对 `'a` 取
  /// `'static` 即可经协变适配任意 builder 生命周期。
  #[inline]
  pub fn owned(bytes: Vec<u8>) -> StringRef<'static> {
    StringRef {
      bytes: Some(Cow::Owned(bytes)),
    }
  }

  /// 底层字节。空槽哨兵（null）读作空切片，与 cpp `string_view(nullptr, 0)` 一致。
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.bytes.as_deref().unwrap_or_default()
  }

  /// 尝试按 UTF-8 解释；Lua 字符串允许任意字节，故这只是读取便利，不是不变量。
  #[inline]
  pub fn as_str(&self) -> Result<&str, Utf8Error> {
    from_utf8(self.as_bytes())
  }

  /// 以 lossy 方式转 `Cow`，仅用于 dump/错误信息等展示路径。
  /// （`Owned` 变体的字节存在 `self` 内，故返回借用只能挂到 `&self` 上。）
  #[inline]
  pub fn to_string_lossy(&self) -> Cow<'_, str> {
    String::from_utf8_lossy(self.as_bytes())
  }

  #[inline]
  pub fn len(&self) -> usize {
    match &self.bytes {
      Some(bytes) => bytes.len(),
      None => 0,
    }
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.bytes.as_deref().is_none_or(<[u8]>::is_empty)
  }

  /// 是否为 cpp 侧的 `data == nullptr` 状态（即 `DenseHashMap` 的空槽哨兵）。
  #[inline]
  pub fn is_null(&self) -> bool {
    self.bytes.is_none()
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
    f.write_str(&self.to_string_lossy())
  }
}

/// C++ `BytecodeBuilder::StringRef::operator==`
/// （`Bytecode/src/BytecodeBuilder.cpp:79`）：
/// `(data && other.data) ? (length == other.length && memcmp(...) == 0) : (data == other.data)`
/// 即两侧非空时比内容，否则比「是否同为 null」——这样 `DenseHashMap` 的 null 空槽哨兵
/// 仍能和自身相等，而空串 `{"", 0}` 是另一个合法键。
impl PartialEq for StringRef<'_> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (&self.bytes, &other.bytes) {
      (Some(lhs), Some(rhs)) => lhs == rhs,
      (None, None) => true,
      _ => false,
    }
  }
}

impl Eq for StringRef<'_> {}

/// C++ `StringRefHash`（`Bytecode/src/BytecodeBuilder.cpp:96`）对内容区间做
/// `hashRange(data, length)`：等内容必须等哈希，`stringTable`/常量表才能按内容去重。
impl Hash for StringRef<'_> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_bytes().hash(state);
  }
}
