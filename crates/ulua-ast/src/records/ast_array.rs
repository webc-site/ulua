#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstArray<T> {
  pub data: *mut T,
  pub size: usize,
}

impl<T> AstArray<T> {
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
    AstArray {
      data: null_mut(),
      size: 0,
    }
  }
}
use core::{
  ffi::c_char,
  ptr::null_mut,
  slice::{Iter, from_raw_parts},
  str::{Utf8Error, from_utf8},
};
