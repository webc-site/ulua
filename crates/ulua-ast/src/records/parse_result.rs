use alloc::vec::Vec;

use crate::{
  functions::optional_node::opt_node,
  records::{
    ast_stat_block::AstStatBlock, comment::Comment, hot_comment::HotComment,
    parse_error::ParseError,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

/// cpp `ParseResult`（`ParseResult.h`）：一次 chunk 解析的完整产物。
///
/// `root` 的可空契约（本结构唯一裸指针字段，写实义如下）：
/// - 写入方只有 [`crate::records::parser::Parser::parse`]，值从 `guarded_parse` 骨架的
///   `ParseNodeResult::root` 原样转写；骨架内部一律用 `Option<NonNull<AstStatBlock>>`
///   表达「有没有根」，只在出口经 [`opt_node`] 折回裸指针。
/// - null 的含义：本次解析在语法错误处放弃了产出根节点（`ParseError` 兜底分支），或
///   单节点形态的 EOF 校验失败使根作废；此时 `errors` 必非空。
/// - 消费方：analysis 把它接到 `SourceModule::root` 并判空（`frontend_parse_*`、
///   `find_*_ast_query`）、compiler 断言非空后编译、unit-test fixture 以 `Option<&…>`
///   暴露。判空是消费方的前置条件，不判空即 UB。
///
/// 字段暂留 `*mut AstStatBlock` 是登记过的卡点：该 pub 字段的读点散布 analysis /
/// compiler / unit-test（>10 文件），且与 `SourceModule::root` 同形态成对，收
/// `Option<NonNull<AstStatBlock>>` 需与那几个 crate 的同一批次推进。
#[derive(Debug, Clone)]
pub struct ParseResult {
  pub root: *mut AstStatBlock,
  pub lines: usize,
  pub hotcomments: Vec<HotComment>,
  pub errors: Vec<ParseError>,
  pub comment_locations: Vec<Comment>,
  pub cst_node_map: CstNodeMap,
}

impl Default for ParseResult {
  /// 「尚未解析」的占位值：无根（null，经 `opt_node` 单源落出）+ 无错误，
  /// 供持有者随后整体覆盖（消费方是 ulua-reduce-cli 的初值）。
  fn default() -> Self {
    Self {
      root: opt_node(None),
      lines: 0,
      hotcomments: Vec::new(),
      errors: Vec::new(),
      comment_locations: Vec::new(),
      cst_node_map: CstNodeMap::default(),
    }
  }
}
