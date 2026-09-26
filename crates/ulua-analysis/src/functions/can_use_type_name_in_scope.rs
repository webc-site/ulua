use crate::{
  records::{scope::Scope, scope_registry::resolve_scope},
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr},
};

pub fn can_use_type_name_in_scope(scope: ScopePtr, name: &str) -> (bool, Option<Name>) {
  let mut curr: Option<&Scope> = Some(&scope);

  while let Some(scope_ref) = curr {
    for (import_name, name_table) in &scope_ref.imported_type_bindings {
      if name_table.contains_key(name) {
        return (true, Some(import_name.clone()));
      }
    }

    if scope_ref.exported_type_bindings.contains_key(name) {
      return (true, None);
    }

    curr = scope_ref.parent.and_then(resolve_scope);
  }

  (false, None)
}
