use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    instantiation_2::Instantiation2, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn instantiate_2(
  arena: *mut TypeArena,
  generic_substitutions: DenseHashMap<TypeId, TypeId>,
  generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
  subtyping: *mut Subtyping,
  scope: *mut Scope,
  ty: TypeId,
) -> Option<TypeId> {
  let mut instantiation =
        Instantiation2::instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
            arena,
            generic_substitutions,
            generic_pack_substitutions,
            subtyping,
            scope,
        );

  instantiation.substitute_type_id(ty)
}
