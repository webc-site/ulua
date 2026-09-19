use alloc::string::String;
use core::{
  cmp::Ordering,
  ffi::{CStr, c_char},
  fmt::{Display, Formatter, Result},
};

/// Port of `Luau::AstName` (Ast/include/Luau/Ast.h:21-60).
///
/// `value` is an interned C string owned by `AstNameTable`; equality/hash are
/// pointer identity (reference `operator==(const AstName&)`), while ordering is
/// content-based `strcmp` with a pointer fallback when either side is null
/// (reference `operator<`). The two are consistent under the interning
/// invariant: equal content from the same table implies the same pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstName {
  pub value: *const c_char,
}

impl PartialOrd for AstName {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AstName {
  fn cmp(&self, other: &Self) -> Ordering {
    if !self.value.is_null() && !other.value.is_null() {
      unsafe { CStr::from_ptr(self.value).cmp(CStr::from_ptr(other.value)) }
    } else {
      self.value.cmp(&other.value)
    }
  }
}

impl AstName {
  /// Returns the name as a CStr, or None if null.
  #[inline]
  pub fn as_c_str(&self) -> Option<&'static CStr> {
    if self.value.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(self.value) })
    }
  }

  /// Returns the name as a byte slice (excluding trailing null byte), or empty slice if null.
  #[inline]
  pub fn as_bytes(&self) -> &'static [u8] {
    match self.as_c_str() {
      Some(c_str) => c_str.to_bytes(),
      None => &[],
    }
  }

  /// Tries to return the name as a UTF-8 `&str`, or None if null or invalid UTF-8.
  #[inline]
  pub fn as_str(&self) -> Option<&'static str> {
    self.as_c_str().and_then(|c_str| c_str.to_str().ok())
  }

  /// Returns the name as a `&str`, or `""` if null or invalid UTF-8.
  #[inline]
  pub fn as_str_or_empty(&self) -> &'static str {
    self.as_str().unwrap_or("")
  }

  /// Creates an AstName from a static CStr reference.
  #[inline]
  pub const fn from_c_str(c_str: &'static CStr) -> Self {
    Self {
      value: c_str.as_ptr(),
    }
  }

  /// Returns the length of the string in bytes.
  #[inline]
  pub fn len(&self) -> usize {
    self.as_bytes().len()
  }

  /// Returns true if the string is empty or null.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns whether this name is null.
  #[inline]
  pub fn is_null(&self) -> bool {
    self.value.is_null()
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
