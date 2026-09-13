/// C++ `static AutocompleteResult autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1568-1580), the six-parameter overload that builds a
/// fresh result.
use alloc::vec::Vec;

use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  enums::autocomplete_context::AutocompleteContext,
  functions::autocomplete_expression_autocomplete_core::autocomplete_expression as autocomplete_expression_into,
  records::{
    autocomplete_result::AutocompleteResult, builtin_types::BuiltinTypes, module::Module,
    type_arena::TypeArena,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr},
};
pub fn autocomplete_expression(
  module: &Module,
  builtin_types: &BuiltinTypes,
  type_arena: *mut TypeArena,
  ancestry: &Vec<*mut AstNode>,
  scope_at_position: &ScopePtr,
  position: Position,
) -> AutocompleteResult {
  let mut result: AutocompleteEntryMap = Default::default();
  let context: AutocompleteContext = autocomplete_expression_into(
    module,
    builtin_types,
    type_arena,
    ancestry,
    scope_at_position,
    position,
    &mut result,
  );
  AutocompleteResult {
    entry_map: result,
    ancestry: ancestry.clone(),
    context,
  }
}
