use crate::{
  functions::{
    can_suggest_inferred_type_autocomplete_core::can_suggest_inferred_type,
    try_to_string_detailed::try_to_string_detailed,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId},
};

pub fn try_get_type_name_in_scope(
  scope: ScopePtr,
  ty: TypeId,
  function_type_arguments: bool,
) -> Option<Name> {
  if !can_suggest_inferred_type(ty) {
    return None;
  }

  try_to_string_detailed(scope, ty, function_type_arguments)
}
