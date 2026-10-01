//! `Location` 字面量简写，`location.rs` / `lexer.rs` / `ast_construct.rs`
//! 各测试 binary 共用。以 `#[path]` 单独编入每个 binary（不集中进 mod.rs，
//! 避免未用到 `loc` 的 binary 报 dead_code）；依赖同目录 `pos.rs`。
#[path = "pos.rs"]
mod pos;

pub use pos::p;
use ulua_ast::records::location::Location;

pub fn loc(bl: u32, bc: u32, el: u32, ec: u32) -> Location {
  Location {
    begin: p(bl, bc),
    end: p(el, ec),
  }
}
