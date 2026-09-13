use std::ptr::null_mut;

use ulua_ast::records::{ast_expr_if_else::AstExprIfElse, ast_node::AstNode, position::Position};

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  records::autocomplete_entry::AutocompleteEntry,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn autocomplete_if_else_expression(
  node: *const AstNode,
  ancestry: &mut [*mut AstNode],
  position: Position,
  out_result: &mut AutocompleteEntryMap,
) -> bool {
  let parent = if ancestry.len() >= 2 {
    *ancestry.get(ancestry.len() - 2).unwrap()
  } else {
    null_mut()
  };

  if parent.is_null() {
    return false;
  }

  if unsafe { (*node).is::<AstExprIfElse>() } {
    return true;
  }

  let if_else_expr = unsafe { (*parent).as_item::<AstExprIfElse>() };
  if if_else_expr.is_null() {
    return false;
  }

  let condition_location = unsafe { (*if_else_expr).condition.as_ref().unwrap().base.location };
  if condition_location.contains_closed(position) {
    return false;
  }

  if !unsafe { (*if_else_expr).has_then } {
    out_result.insert(
      "then".to_string(),
      autocomplete_entry_kind_to_autocomplete_entry(AutocompleteEntryKind::Keyword),
    );
    return true;
  }

  let true_expr = unsafe { (*if_else_expr).true_expr };
  let true_expr_location = unsafe { (*true_expr).base.location };
  if true_expr_location.contains_closed(position) {
    return false;
  }

  if !unsafe { (*if_else_expr).has_else } {
    out_result.insert(
      "else".to_string(),
      autocomplete_entry_kind_to_autocomplete_entry(AutocompleteEntryKind::Keyword),
    );
    out_result.insert(
      "elseif".to_string(),
      autocomplete_entry_kind_to_autocomplete_entry(AutocompleteEntryKind::Keyword),
    );
    return true;
  }

  false
}

fn autocomplete_entry_kind_to_autocomplete_entry(kind: AutocompleteEntryKind) -> AutocompleteEntry {
  AutocompleteEntry {
    kind,
    ..Default::default()
  }
}
