//! 位置字面量简写，`location.rs` / `ast_construct.rs` / `cst_positions.rs`
//! 各测试 binary 共用（按 binary 单独编入，见同目录 `loc.rs`）。
use ulua_ast::records::position::Position;

pub fn p(line: u32, column: u32) -> Position {
  Position { line, column }
}
