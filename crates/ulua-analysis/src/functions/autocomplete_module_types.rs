use ulua_ast::records::position::Position;

use crate::{
  enums::{
    autocomplete_entry_kind::AutocompleteEntryKind,
    parentheses_recommendation::ParenthesesRecommendation, type_correct_kind::TypeCorrectKind,
  },
  records::{autocomplete_entry::AutocompleteEntry, module::Module},
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr, tags::Tags,
  },
};
pub fn autocomplete_module_types(
  _module: &Module,
  scope_at_position: &ScopePtr,
  _position: Position,
  module_name: &str,
) -> AutocompleteEntryMap {
  let mut result = AutocompleteEntryMap::new();
  let mut curr = Some(scope_at_position.clone());

  while let Some(scope_ref) = curr {
    if let Some(name_table) = scope_ref.imported_type_bindings.get(module_name) {
      for (name, ty) in name_table {
        let entry = AutocompleteEntry {
          kind: AutocompleteEntryKind::Type,
          r#type: Some(ty.r#type),
          deprecated: false,
          wrong_index_type: false,
          type_correct: TypeCorrectKind::None,
          containing_extern_type: None,
          prop: None,
          documentation_symbol: None,
          tags: Tags::default(),
          parens: ParenthesesRecommendation::None,
          insert_text: None,
          indexed_with_self: false,
        };
        result.insert(name.clone(), entry);
      }
      break;
    }

    curr = scope_ref.parent.clone();
  }

  result
}
