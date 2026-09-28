use ulua_ast::records::position::Position;

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  records::{
    autocomplete_entry::AutocompleteEntry, module::Module, scope::Scope,
    scope_registry::resolve_scope,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr},
};
pub fn autocomplete_module_types(
  _module: &Module,
  scope_at_position: &ScopePtr,
  _position: Position,
  module_name: &str,
) -> AutocompleteEntryMap {
  let mut result = AutocompleteEntryMap::new();
  let mut curr: Option<&Scope> = Some(scope_at_position.as_ref());

  while let Some(scope_ref) = curr {
    if let Some(name_table) = scope_ref.imported_type_bindings.get(module_name) {
      for (name, ty) in name_table {
        let entry = AutocompleteEntry {
          kind: AutocompleteEntryKind::Type,
          r#type: Some(ty.r#type),
          ..Default::default()
        };
        result.insert(name.clone(), entry);
      }
      break;
    }

    curr = scope_ref.parent.and_then(resolve_scope);
  }

  result
}
