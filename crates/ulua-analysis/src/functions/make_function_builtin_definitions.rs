use alloc::vec::Vec;

use crate::{
  functions::make_function_builtin_definitions_alt_b::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool,
  records::type_arena::TypeArena, type_aliases::type_id::TypeId,
};

pub fn make_function(
  arena: &mut TypeArena,
  self_type: Option<TypeId>,
  param_types: Vec<TypeId>,
  ret_types: Vec<TypeId>,
  checked: bool,
) -> TypeId {
  make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool(arena, self_type, Vec::new(), Vec::new(), param_types, ret_types, checked)
}
