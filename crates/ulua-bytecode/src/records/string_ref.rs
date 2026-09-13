use alloc::{borrow::Cow, string::String};
use core::{
  convert::Infallible,
  ffi::c_char,
  fmt::{self, Display, Formatter},
  hash::{Hash, Hasher},
  slice,
  str::{FromStr, Utf8Error, from_utf8},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct StringRef {
  pub(crate) data: *const u8,
  pub(crate) length: usize,
}

impl StringRef {
  pub fn new(data: *const c_char, length: usize) -> Self {
    Self {
      data: data as *const u8,
      length,
    }
  }

  /// Construct a `StringRef` from a byte slice.
  #[inline]
  pub fn from_slice(bytes: &[u8]) -> Self {
    Self {
      data: bytes.as_ptr(),
      length: bytes.len(),
    }
  }

  /// Returns the underlying byte slice.
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    if self.data.is_null() {
      &[]
    } else {
      unsafe { slice::from_raw_parts(self.data, self.length) }
    }
  }

  /// Tries to convert the string ref into a UTF-8 `&str`.
  #[inline]
  pub fn as_str(&self) -> Result<&str, Utf8Error> {
    from_utf8(self.as_bytes())
  }

  /// Converts the string ref to a Cow string, replacing invalid UTF-8 sequences.
  #[inline]
  pub fn to_string_lossy(&self) -> Cow<'_, str> {
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

impl<'a> From<&'a [u8]> for StringRef {
  #[inline]
  fn from(bytes: &'a [u8]) -> Self {
    Self::from_slice(bytes)
  }
}

impl<'a> From<&'a str> for StringRef {
  #[inline]
  fn from(s: &'a str) -> Self {
    Self::from_slice(s.as_bytes())
  }
}

impl FromStr for StringRef {
  type Err = Infallible;

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self::from_slice(s.as_bytes()))
  }
}

impl Display for StringRef {
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
impl PartialEq for StringRef {
  fn eq(&self, other: &Self) -> bool {
    if !self.data.is_null() && !other.data.is_null() {
      self.length == other.length && self.as_bytes() == other.as_bytes()
    } else {
      self.data == other.data
    }
  }
}

impl PartialEq<&str> for StringRef {
  #[inline]
  fn eq(&self, other: &&str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<StringRef> for &str {
  #[inline]
  fn eq(&self, other: &StringRef) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<str> for StringRef {
  #[inline]
  fn eq(&self, other: &str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl Eq for StringRef {}

// C++ `StringRefHash` hashes the content range (`hashRange(data, length)`); equal
// content must hash equally for the dedup map.
impl Hash for StringRef {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_bytes().hash(state);
  }
}
