//! Faithful port of Luau `Location` (`Ast/include/Luau/Location.h`).
//!
//! A half-open source span `begin..end`. C++ defines `operator==`/`!=` (collapse
//! into the derived `PartialEq`/`Eq`) but no ordering. `Default` is the
//! all-zero span, matching the C++ `Location()` default constructor. The four
//! `Location(...)` constructors and the `encloses`/`overlaps`/`contains`/
//! `extend`/`shift` methods are separate items.

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::records::position::Position;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Location {
  pub begin: Position,
  pub end: Position,
}

// Usable as a `DenseHashMap` value (e.g. `Parser::declared_export_bindings`); the
// empty slot is the all-zero span, matching C++ value-initialization.
impl DenseDefault for Location {
  fn dense_default() -> Self {
    Location::default()
  }
}
