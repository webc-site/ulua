use alloc::vec::Vec;

use crate::{
  records::{
    ast_node::AstNode, comment::Comment, hot_comment::HotComment, parse_error::ParseError,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

/// 单节点形态的解析产物（cpp 模板 `ParseResultT<T>`），`guarded_parse` 骨架的返回类型。
///
/// `root` 的可空语义与 [`crate::records::parse_result::ParseResult::root`] 完全同源
/// （骨架内以 `Option<NonNull>` 表达「有无根」，出口经 `opt_node` 折回；null 表示语法
/// 错误兜底或 EOF 校验令根作废），字段为何暂留裸指针的理由见该类型的契约说明。
#[derive(Debug, Clone)]
pub struct ParseNodeResult<Node = AstNode> {
  pub root: *mut Node,
  pub lines: usize,
  pub hotcomments: Vec<HotComment>,
  pub errors: Vec<ParseError>,
  pub comment_locations: Vec<Comment>,
  pub cst_node_map: CstNodeMap,
}
