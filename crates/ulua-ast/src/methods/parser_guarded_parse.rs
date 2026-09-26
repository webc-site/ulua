//! `Parser::parse` 与 `Parser::run_parse` 共用的驱动骨架 —— cpp
//! `Parser::parse`（`Ast/src/Parser.cpp:226`）与模板 `Parser::runParse`
//! （`:248`）里同一段 `try { ... } catch (ParseError& err)`。
//!
//! 两处差异只有：根节点怎么来（`parseChunk()` vs 调用方传入的 `f`）、以及
//! `runParse` 多出的 EOF 校验；因此骨架收敛到本函数，差异以 `body` /
//! `finish` 两个闭包参数表达（`finish` 在 cpp 中位于行数计算之后，顺序照抄）。

use core::ptr::NonNull;
use std::{
  mem::take,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::{
    install_parse_error_panic_hook::install_parse_error_panic_hook, optional_node::opt_node,
  },
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
  ///
  /// `finish` 返回 `Option<NonNull<Node>>`：`None` 表示「根节点作废」（cpp 把 `root`
  /// 重置为 `nullptr` 并压一条错误），驱动骨架在出口用 [`opt_node`] 折回 `root` 字段的
  /// 裸指针形态——`ParseNodeResult::root` 本体尚未去哨兵化（消费面散布 analysis），故
  /// 本函数内部的「有无根」一律用 `Option` 表达。
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
    Finish: FnOnce(&mut Parser, *mut Node) -> Option<NonNull<Node>>,
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
        root: opt_node(root),
        lines: lines as usize,
        hotcomments: take(&mut p.hotcomments),
        errors: take(&mut p.parse_errors),
        comment_locations: take(&mut p.comment_locations),
        cst_node_map: take(&mut p.cst_node_map),
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
              // 抛错的解析没有根节点：`None` 经 `opt_node` 落成 cpp 的 `root = nullptr`。
              root: opt_node(None),
              lines: 0,
              hotcomments: Vec::new(),
              errors: take(&mut p.parse_errors),
              comment_locations: Vec::new(),
              cst_node_map: take(&mut p.cst_node_map),
            }
          }
          // 非 ParseError 的 panic 不是语法错误，原样上抛
          Err(payload) => resume_unwind(payload),
        }
      }
    }
  }
}
