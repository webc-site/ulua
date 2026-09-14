//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Set.h:132:const_iterator`
//! Source: `Analysis/include/Luau/Set.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Set.h
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Set.h
//!   - type_ref <- record Set (Analysis/include/Luau/Set.h)
//!   - type_ref <- type_alias iterator (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::begin (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::end (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::const_iterator::const_iterator (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::const_iterator::operator== (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::const_iterator::operator!= (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::const_iterator::operator++ (Analysis/include/Luau/Set.h)
//!   - type_ref <- method Set::const_iterator::operator++ (Analysis/include/Luau/Set.h)
//!   - type_ref <- type_alias const_iterator (Analysis/include/Luau/TypeIds.h)
//! - outgoing:
//!   - type_ref -> type_alias const_iterator (Analysis/include/Luau/TypeIds.h)
//!   - translates_to -> rust_item const_iterator

use core::fmt::{Debug, Formatter, Result};
extern crate alloc;

use alloc::boxed::Box;

// C++ Set<T>::const_iterator (Set.h:132-191): forward iteration over the
// underlying DenseHashMap<T, bool>, skipping tombstoned (false) entries. The
// skip-false filter is applied where Set::begin constructs this (the C++ ctor
// and operator++ both skip); the boxed inner iterator carries it.
pub struct ConstIterator<'a, T> {
  pub(crate) inner: Box<dyn Iterator<Item = &'a T> + 'a>,
}

impl<'a, T> ConstIterator<'a, T> {
  pub fn new(inner: Box<dyn Iterator<Item = &'a T> + 'a>) -> Self {
    Self { inner }
  }
}

impl<'a, T> Iterator for ConstIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<&'a T> {
    self.inner.next()
  }
}

impl<T> Debug for ConstIterator<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str("ConstIterator")
  }
}
