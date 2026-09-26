//! Faithful port of Luau `Location` (`Ast/include/Luau/Location.h`).
//!
//! A half-open source span `begin..end`. C++ defines `operator==`/`!=` (collapse
//! into the derived `PartialEq`/`Eq`) but no ordering. `Default` is the
//! all-zero span, matching the C++ `Location()` default constructor.

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

impl Location {
  pub const fn new(begin: Position, end: Position) -> Self {
    Self { begin, end }
  }

  /// `end = Position(begin.line, begin.column + length)`.
  pub const fn with_length(begin: Position, length: u32) -> Self {
    Self {
      begin,
      end: Position {
        line: begin.line,
        column: begin.column + length,
      },
    }
  }

  /// Spans from one location's start to another's end.
  pub const fn between(begin: Location, end: Location) -> Self {
    Self {
      begin: begin.begin,
      end: end.end,
    }
  }

  pub fn contains(&self, position: Position) -> bool {
    self.begin <= position && position < self.end
  }

  pub fn contains_closed(&self, position: Position) -> bool {
    self.begin <= position && position <= self.end
  }

  pub fn encloses(&self, other: &Location) -> bool {
    self.begin <= other.begin && other.end <= self.end
  }

  pub fn overlaps(&self, other: &Location) -> bool {
    (self.begin <= other.begin && self.end >= other.begin)
      || (self.begin <= other.end && self.end >= other.end)
      || (self.begin >= other.begin && self.end <= other.end)
  }

  pub fn extend(&mut self, next: &Location) {
    if next.begin < self.begin {
      self.begin = next.begin;
    }
    if self.end < next.end {
      self.end = next.end;
    }
  }

  pub fn shift(&mut self, start: &Position, old_end: &Position, new_end: &Position) {
    self.begin.shift(start, old_end, new_end);
    self.end.shift(start, old_end, new_end);
  }

  /// 依据行列偏移平移区间。
  pub fn shift_offset(&mut self, offset: Position) {
    self.begin.shift_offset(offset);
    self.end.shift_offset(offset);
  }
}
