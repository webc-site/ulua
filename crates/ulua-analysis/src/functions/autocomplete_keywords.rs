use ulua_ast::{
  records::{ast_expr_function::AstExprFunction, ast_node::AstNode, position::Position},
  rtti::{ast_node_is, is_expr_class},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  records::autocomplete_entry::AutocompleteEntry,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

/// 插入一个纯关键字补全项：除 `kind` 外全部字段即 `AutocompleteEntry` 的默认值
/// （对应 cpp `AutocompleteEntry{/*kind=*/Keyword{}}` 的零初始化）。
pub(crate) fn insert_keyword(result: &mut AutocompleteEntryMap, kw: &str) {
  result.insert(
    kw.to_string(),
    AutocompleteEntry {
      kind: AutocompleteEntryKind::Keyword,
      ..Default::default()
    },
  );
}

pub fn autocomplete_keywords(
  ancestry: &[*mut AstNode],
  _position: Position,
  result: &mut AutocompleteEntryMap,
) {
  LUAU_ASSERT!(!ancestry.is_empty());

  // 紧邻 LUAU_ASSERT(!ancestry.is_empty()) 蕴含 last() 命中 Some。
  let node = *ancestry
    .last()
    .expect("上方 LUAU_ASSERT(!ancestry.is_empty()) 蕴含非空");

  // Safety: ancestry 由 autocomplete 沿已解析语法树逐层下钻记录，元素恒为非空
  // AstNode*（parser 结构性保证子节点指针非空，根节点由 caller 传入），指向
  // parser arena 中存活节点——块地址不移动，AST 于补全查询期内整体存活；函数头
  // 一次重建只读借用后，ast_node_is 与 class_index 均为不可变访问，无别名冲突。
  let node_ref = unsafe { &*node };

  let is_expr_function = ast_node_is::<AstExprFunction>(node_ref);
  let is_expr = is_expr_class(node_ref.class_index);

  if !is_expr_function && is_expr {
    for kw in ["and", "or", "not"] {
      insert_keyword(result, kw);
    }
  }
}
