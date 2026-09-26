//! `Fixture::try_parse` / `Fixture::parse_ex` 的返回包装：除 `ParseResult` 外
//! 附带借用绑定到 fixture 的根块引用，调用侧免 unsafe 解引用。
//!
//! `Deref<Target = ParseResult>` 保持 `result.errors`、`result.root`（裸指针
//! 字段，供 `result.root as *mut AstNode` 等指针场景）的原有访问形态；
//! 根块解引用统一走 `root_block()`。

use core::ops::Deref;

use ulua_ast::records::{ast_stat_block::AstStatBlock, parse_result::ParseResult};

pub struct TryParseResult<'a> {
  /// 原始 ParseResult（root 仍为裸指针字段，供指针形态使用）
  pub result: ParseResult,
  /// 与 fixture 借用绑定的根块引用；硬错误路径（Parser::parse 的单条
  /// ParseError unwind 返回 null root）为 None，对应 C++ `root == nullptr`。
  pub(crate) root: Option<&'a AstStatBlock>,
}

impl<'a> TryParseResult<'a> {
  /// Safety 封装：非 null root 指向 fixture.allocator 中存活的根块。
  pub fn root_block(&self) -> Option<&'a AstStatBlock> {
    self.root
  }
}

impl Deref for TryParseResult<'_> {
  type Target = ParseResult;

  fn deref(&self) -> &ParseResult {
    &self.result
  }
}
