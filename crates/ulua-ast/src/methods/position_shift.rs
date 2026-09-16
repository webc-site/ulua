use crate::records::position::Position;

impl Position {
  pub fn shift(&mut self, start: &Position, old_end: &Position, new_end: &Position) {
    if self.position_operator_ge(start) {
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

pub fn position_shift(
  position: &mut Position,
  start: &Position,
  old_end: &Position,
  new_end: &Position,
) {
  position.shift(start, old_end, new_end);
}
