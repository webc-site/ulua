use core::ptr::null_mut;

use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, vec_deque::VecDeque,
};

use crate::records::{
  builtin_types::BuiltinTypes, normalizer::Normalizer, type_arena::TypeArena,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};
impl TypeFunctionReductionGuesser {
  pub fn type_function_reduction_guesser_type_function_reduction_guesser(
    arena: *mut TypeArena,
    builtins: *mut BuiltinTypes,
    normalizer: *mut Normalizer,
  ) -> Self {
    Self {
      function_reduces_to: DenseHashMap::new(null_mut()),
      substitutable: DenseHashMap::new(null_mut()),
      to_infer: VecDeque::new(),
      cyclic_instances: DenseHashSet::new(null_mut()),
      arena,
      builtins,
      normalizer,
    }
  }
}
