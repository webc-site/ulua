//! `Parser::parse` 与 `Parser::run_parse` 共用的驱动骨架 —— cpp
//! `Parser::parse`（`Ast/src/Parser.cpp:226`）与模板 `Parser::runParse`
//! （`:248`）里同一段 `try { ... } catch (ParseError& err)`。
//!
//! 两处差异只有：根节点怎么来（`parseChunk()` vs 调用方传入的 `f`）、以及
//! `runParse` 多出的 EOF 校验；因此骨架收敛到本函数，差异以 `body` /
//! `finish` 两个闭包参数表达（`finish` 在 cpp 中位于行数计算之后，顺序照抄）。

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
    parse_node_result::ParseNodeResult, parse_options::ParseOptions, parser::Parser,
  },
};

impl Parser {
  /// 建 parser、跑一次解析、把 `ParseError` panic（cpp `throw ParseError`）在
  /// 边界处收敛为「空根 + 错误表」结果，其余 panic 原样上抛。
  ///
  /// `buffer` 是原始源码字节：Luau 词法器按字节工作，源缓冲不要求是合法
  /// UTF-8；长度即 `buffer.len()`，没有独立可漂移的 size 参数。
  pub(crate) fn guarded_parse<Node, Body, Finish>(
    buffer: &[u8],
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
    body: Body,
    finish: Finish,
  ) -> ParseNodeResult<Node>
  where
    Body: FnOnce(&mut Parser) -> *mut Node,
    Finish: FnOnce(&mut Parser, *mut Node) -> *mut Node,
  {
    LUAU_TIMETRACE_SCOPE!("Parser::parse", "Parser");

    // Silence the default panic-hook noise for the parser's exception-
    // emulation unwinds (a caught `ParseError` is a normal syntax/limit
    // error, not a crash).
    install_parse_error_panic_hook();

    let mut p = Parser::new(buffer, names, allocator as *mut Allocator, options);

    let result = catch_unwind(AssertUnwindSafe(|| {
      let root = body(&mut p);

      // cpp: `p.lexer.current().location.end.line + (bufferSize > 0 && buffer[bufferSize - 1] != '\n')`
      // —— 空缓冲不计数，故末字节判断必须先排除 empty。
      let mut lines = p.lexer.current().location.end.line;
      if !buffer.is_empty() && buffer.last() != Some(&b'\n') {
        lines += 1;
      }

      let root = finish(&mut p, root);

      ParseNodeResult {
        root,
        lines: lines as usize,
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

            ParseNodeResult {
              root: null_mut(),
              lines: 0,
              hotcomments: Vec::new(),
              errors: take(&mut p.parse_errors),
              comment_locations: Vec::new(),
              cst_node_map: replace(&mut p.cst_node_map, DenseHashMap::new(null_mut())),
            }
          }
          // 非 ParseError 的 panic 不是语法错误，原样上抛
          Err(payload) => resume_unwind(payload),
        }
      }
    }
  }
}
