use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, vec_deque::VecDeque,
};

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, normalizer::Normalizer, type_arena::TypeArena,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};
impl TypeFunctionReductionGuesser {
  pub fn type_function_reduction_guesser_type_function_reduction_guesser(
    arena: Handle<TypeArena>,
    builtins: Handle<BuiltinTypes>,
    normalizer: *mut Normalizer,
  ) -> Self {
    Self {
      function_reduces_to: DenseHashMap::default(),
      substitutable: DenseHashMap::default(),
      to_infer: VecDeque::new(),
      cyclic_instances: DenseHashSet::default(),
      arena,
      builtins,
      normalizer,
    }
  }
}
