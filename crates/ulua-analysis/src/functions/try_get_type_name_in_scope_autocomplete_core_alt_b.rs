use crate::{
  functions::{
    can_suggest_inferred_type_autocomplete_core_alt_b::can_suggest_inferred_type_type_pack_id,
    try_to_string_detailed::try_to_string_detailed,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

pub fn try_get_type_name_in_scope_scope_ptr_type_pack_id_bool(
  scope: ScopePtr,
  tp: TypePackId,
  function_type_arguments: bool,
) -> Option<Name> {
  if !can_suggest_inferred_type_type_pack_id(tp) {
    return None;
  }

  try_to_string_detailed(scope, tp, function_type_arguments)
}
