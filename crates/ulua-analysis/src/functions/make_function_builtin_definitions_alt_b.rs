use alloc::vec::Vec;

use crate::{
  functions::make_function_builtin_definitions_alt_d::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool,
  records::type_arena::TypeArena,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool(
  arena: &mut TypeArena,
  self_type: Option<TypeId>,
  generics: Vec<TypeId>,
  generic_packs: Vec<TypePackId>,
  param_types: Vec<TypeId>,
  ret_types: Vec<TypeId>,
  checked: bool,
) -> TypeId {
  make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool(arena, self_type, generics, generic_packs, param_types, Vec::new(), ret_types, checked)
}
