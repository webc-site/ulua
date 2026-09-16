/// C++ `autocompleteStringSingleton`（Autocomplete.cpp）：把 string singleton、
/// string 的 union/intersection 展开为补全候选键。
use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;
use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_node::AstNode, position::Position,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::{FFlag, functions::escape::escape};

use crate::{
  enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
  functions::{
    follow_type::follow_type_id, get_singleton_type::get_singleton_type,
    get_type_alt_j::get_type_id,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, intersection_type::IntersectionType,
    singleton_type::SingletonType, string_singleton::StringSingleton, union_type::UnionType,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, type_id::TypeId},
};
pub fn autocomplete_string_singleton(
  ty: TypeId,
  add_quotes: bool,
  node: &AstNode,
  position: Position,
  result: &mut AutocompleteEntryMap,
) {
  if position == node.location.begin || position == node.location.end {
    if let Some(str_val) = ast_node_try_as::<AstExprConstantString>(node)
      && str_val.is_quoted()
    {
      return;
    }

    if ast_node_is::<AstExprInterpString>(node) {
      return;
    }
  }

  let format_key = |key: &str| {
    if add_quotes {
      format!("\"{}\"", escape(key, false))
    } else {
      escape(key, false)
    }
  };

  let ty = follow_type_id(ty);
  let entry_for = |key: String, result: &mut AutocompleteEntryMap| {
    result.entry(key).or_insert_with(|| AutocompleteEntry {
      kind: AutocompleteEntryKind::String,
      r#type: Some(ty),
      deprecated: false,
      wrong_index_type: false,
      type_correct: TypeCorrectKind::Correct,
      containing_extern_type: None,
      prop: None,
      documentation_symbol: None,
      tags: Default::default(),
      parens: Default::default(),
      insert_text: None,
      indexed_with_self: false,
    });
  };

  let ss = get_type_id::<SingletonType>(ty);
  if let Some(ss) = ss {
    if let Some(sstv) = get_singleton_type::<StringSingleton>(ss) {
      entry_for(format_key(&sstv.value), result);
    }
    return;
  }

  if let Some(uty) = get_type_id::<UnionType>(ty) {
    for &el in &uty.options {
      if let Some(ss_el) = get_type_id::<SingletonType>(el)
        && let Some(sstv) = get_singleton_type::<StringSingleton>(ss_el)
      {
        entry_for(format_key(&sstv.value), result);
      }
    }
    return;
  }

  let ity = get_type_id::<IntersectionType>(ty);
  if FFlag::LuauAutocompleteStringSingletonIntersection.get()
    && let Some(itv) = ity
  {
    for &el in &itv.parts {
      autocomplete_string_singleton(el, add_quotes, node, position, result);
    }
  }
}
