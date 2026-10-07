//! Faithful port of Luau `Position` (`Ast/include/Luau/Location.h`).
//!
//! `unsigned int line, column`. C++ `operator<` orders by line then column,
//! which is exactly what the derived `Ord` does over the fields in declaration
//! order — so the six comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
//! collapse into the derives here, and their separate method items stay stubs.
//! `Default` is `(0, 0)`, matching the `Position(0, 0)` used by `Location()`.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
  pub line: u32,
  pub column: u32,
}

impl Position {
  /// cpp `Position()` 的「无位置」哨兵分量：`line`/`column` 皆取 `u32::MAX`。
  const MISSING: u32 = u32::MAX;

  pub const fn new(line: u32, column: u32) -> Self {
    Self { line, column }
  }

  pub const fn missing() -> Self {
    Self {
      line: Self::MISSING,
      column: Self::MISSING,
    }
  }

  pub const fn has_value(&self) -> bool {
    self.line != Self::MISSING || self.column != Self::MISSING
  }

  pub fn shift(&mut self, start: &Position, old_end: &Position, new_end: &Position) {
    if *self >= *start {
      // C++ 的 unsigned 算术回绕；Rust 普通加减在 debug 下会 panic，
      // 用 wrapping 保持与 C++ 完全一致（缩短替换时 column 会下溢）。
      if self.line > old_end.line {
        self.line = self
          .line
          .wrapping_add(new_end.line.wrapping_sub(old_end.line));
      } else {
        self.line = new_end.line;
        self.column = self
          .column
          .wrapping_add(new_end.column.wrapping_sub(old_end.column));
      }
    }
  }
}

/// 空位置切片（cpp `AstArray<Position>{}` / nullptr 哨兵的等价形态，
/// CommaSeparatorInserter 等处复用）。
pub(crate) const EMPTY_POSITIONS: &[Position] = &[];
