//! Source: `Analysis/include/Luau/Set.h`

use core::fmt::{Debug, Formatter, Result};

// C++ Set<T>::const_iterator (Set.h:132-191): forward iteration over the
// underlying DenseHashMap<T, bool>, skipping tombstoned (false) entries. The
// skip-false filter is applied where Set::begin constructs this (the C++ ctor
// and operator++ both skip); the inner iterator carries it.
pub struct ConstIterator<'a, T: 'a, I: Iterator<Item = &'a T>> {
  pub(crate) inner: I,
}

impl<'a, T: 'a, I: Iterator<Item = &'a T>> ConstIterator<'a, T, I> {
  pub fn new(inner: I) -> Self {
    Self { inner }
  }
}

impl<'a, T: 'a, I: Iterator<Item = &'a T>> Iterator for ConstIterator<'a, T, I> {
  type Item = &'a T;

  fn next(&mut self) -> Option<&'a T> {
    self.inner.next()
  }
}

impl<T: 'static, I: Iterator<Item = &'static T>> Debug for ConstIterator<'static, T, I> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str("ConstIterator")
  }
}
