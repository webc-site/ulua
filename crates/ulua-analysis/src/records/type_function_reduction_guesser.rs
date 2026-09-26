use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, vec_deque::VecDeque,
};

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, normalizer::Normalizer,
    type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionReductionGuesser {
  pub(crate) function_reduces_to: DenseHashMap<TypeId, TypeId>,
  pub(crate) substitutable: DenseHashMap<TypeId, TypeId>,
  pub(crate) to_infer: VecDeque<TypeId>,
  pub(crate) cyclic_instances: DenseHashSet<TypeId>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) builtins: Handle<BuiltinTypes>,
  pub(crate) normalizer: *mut Normalizer,
}
