//! `Lexer::Lexer(const char* buffer, size_t buffer_size, AstNameTable& names, Position startPosition)`
//! — Ast/src/Lexer.cpp:346.

use alloc::vec::Vec;
use core::{ptr::from_mut, slice::from_raw_parts};

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_name_table::AstNameTable, lexeme::Lexeme, lexer::Lexer, location::Location,
    position::Position,
  },
};

impl Lexer {
  /// 词法器构造入口（cpp 四参构造器的 Rust 形态）。
  ///
  /// 契约：`buffer` 必须存活到返回的 `Lexer` 及其产出的全部词素被消费完毕
  /// （cpp 侧同样只保存 `const char*` + size 的非拥有视图，词素 `data` 臂存
  /// `&buffer[startOffset]`；`Parser::run_parse`/`guarded_parse` 的调用点均满足）。
  pub fn new(buffer: &[u8], names: &mut AstNameTable, start_position: Position) -> Lexer {
    // Safety: 上方契约即前置条件——源缓冲活过整个解析会话（与 cpp 引用参数
    // `AstNameTable&`/裸 `buffer` 的存活要求同源）；把借用切片锻造为 `&'static`
    // 只延长类型上的生命周期标注，不改变任何字节的读取界（全部读路径以
    // `offset < buffer.len()` 判界），入参借用本身不在函数返回值中留存。
    let buffer: &'static [u8] = unsafe { from_raw_parts(buffer.as_ptr(), buffer.len()) };

    Lexer {
      buffer,
      offset: 0,
      line: start_position.line,
      // `lineOffset(0u - startPosition.column)` — wrapping so that
      // `position()` reports `offset - lineOffset` == the start column.
      line_offset: 0u32.wrapping_sub(start_position.column),
      lexeme: Lexeme::new(
        Location::with_length(
          Position {
            line: start_position.line,
            column: start_position.column,
          },
          0,
        ),
        Type::EOF,
      ),
      prev_location: Location::default(),
      // `&mut → *mut` 地址视图（Box 钉堆：names 宿主地址稳定，cpp 引用成员
      // 的等价形态），无 `as` 强转。
      names: from_mut(names),
      skip_comments: false,
      read_names: true,
      brace_stack: Vec::new(),
    }
  }
}
