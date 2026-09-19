use core::{mem::replace, ptr::null_mut};
use std::{
  mem::take,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_common::{
  macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE, records::dense_hash_map::DenseHashMap,
};

use crate::{
  functions::install_parse_error_panic_hook::install_parse_error_panic_hook,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_error::ParseError,
    parse_options::ParseOptions, parse_result::ParseResult, parser::Parser,
  },
};

impl Parser {
  /// `buffer` 是原始源码字节：Luau 的词法器按字节工作，源文件不要求是合法
  /// UTF-8（`"\xFF"` 这类字面量允许出现在任意位置）。
  pub fn parse<B>(
    buffer: &B,
    buffer_size: usize,
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
  ) -> ParseResult
  where
    B: AsRef<[u8]> + ?Sized,
  {
    LUAU_TIMETRACE_SCOPE!("Parser::parse", "Parser");

    // Silence the default panic-hook noise for the parser's exception-
    // emulation unwinds (a caught `ParseError` is a normal syntax/limit
    // error, not a crash).
    install_parse_error_panic_hook();

    let mut p = Parser::new(buffer, names, allocator as *mut Allocator, options);

    let result = catch_unwind(AssertUnwindSafe(|| {
      let root = p.parse_chunk();
      let current_lexeme = p.lexer.current();
      let mut line_count = current_lexeme.location.end.line;
      if buffer_size > 0 && buffer.as_ref()[buffer_size - 1] != b'\n' {
        line_count += 1;
      }
      let lines = line_count as usize;

      ParseResult {
        root,
        lines,
        hotcomments: take(&mut p.hotcomments),
        errors: take(&mut p.parse_errors),
        comment_locations: take(&mut p.comment_locations),
        cst_node_map: replace(&mut p.cst_node_map, DenseHashMap::new(null_mut())),
      }
    }));

    match result {
      Ok(res) => res,
      Err(payload) => {
        // downcast 按值取回 Box<ParseError>，直接移动进错误表，省一次 clone
        match payload.downcast::<ParseError>() {
          Ok(err) => {
            p.parse_errors.push(*err);

            ParseResult {
              root: null_mut(),
              lines: 0,
              hotcomments: Vec::new(),
              errors: take(&mut p.parse_errors),
              comment_locations: Vec::new(),
              cst_node_map: replace(&mut p.cst_node_map, DenseHashMap::new(null_mut())),
            }
          }
          Err(payload) => resume_unwind(payload),
        }
      }
    }
  }
}
