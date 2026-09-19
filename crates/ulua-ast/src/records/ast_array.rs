use core::{
  ffi::c_char,
  ptr::null_mut,
  slice::{Iter, from_raw_parts},
  str::{Utf8Error, from_utf8},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstArray<T> {
  pub data: *mut T,
  pub size: usize,
}

impl<T> AstArray<T> {
  /// 空数组（`{nullptr, 0}`），对应 C++ `AstArray<T>{}`；替代调用点手写字面量。
  pub const EMPTY: Self = Self {
    data: null_mut(),
    size: 0,
  };

  /// The elements as a slice. C++ `AstArray` exposes `begin()`/`end()` over
  /// `[data, data + size)`; the arena keeps the backing storage alive.
  ///
  /// # Safety note
  /// `data`/`size` come from the arena allocator and are always a valid region
  /// (or `data == null` with `size == 0`), so this is sound for live nodes.
  pub fn as_slice(&self) -> &[T] {
    if self.data.is_null() {
      &[]
    } else {
      unsafe { from_raw_parts(self.data, self.size) }
    }
  }

  pub fn iter(&self) -> Iter<'_, T> {
    self.as_slice().iter()
  }

  pub fn len(&self) -> usize {
    self.size
  }

  pub fn is_empty(&self) -> bool {
    self.size == 0
  }
}

impl<T> AstArray<*mut T> {
  /// 节点指针数组的可变引用迭代器：元素裸指针逐个解引用为 `&mut T`
  /// （cpp `for (auto o : a->generics)` 遍历指针形态的安全对应）。
  ///
  /// 前提（visualize 打印器遍历场景成立）：元素均指向 arena 存活节点，同一
  /// 节点的 `&mut` 生命周期互不重叠——arena 独占所有权，逐个用完即弃。
  #[inline]
  pub fn iter_mut_nodes(&self) -> impl Iterator<Item = &'_ mut T> {
    self.as_slice().iter().map(|&p| unsafe { &mut *p })
  }
}

impl AstArray<c_char> {
  /// Returns the backing `c_char` buffer as a byte slice `&[u8]`.
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    if self.data.is_null() || self.size == 0 {
      &[]
    } else {
      unsafe { from_raw_parts(self.data as *const u8, self.size) }
    }
  }

  /// Tries to convert the backing `c_char` buffer to a UTF-8 `&str`.
  #[inline]
  pub fn as_str(&self) -> Result<&str, Utf8Error> {
    from_utf8(self.as_bytes())
  }

  /// 是否含 NUL 字节：AstName 承载的 `char*` 不能指向 NUL（C++
  /// `strchr(..., 0)` 前置判定），原 parser 两处手动循环合并于此。
  #[inline]
  pub fn contains_null(&self) -> bool {
    self.as_bytes().contains(&0)
  }
}

impl<'a, T> IntoIterator for &'a AstArray<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

// An empty array (`{nullptr, 0}`) for every `T`, mirroring C++ `AstArray<T>{}`.
// Manual (not derived) so it does not require `T: Default`.
impl<T> Default for AstArray<T> {
  fn default() -> Self {
    AstArray::EMPTY
  }
}
