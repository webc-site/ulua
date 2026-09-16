use alloc::vec::Vec;
use core::ptr::{NonNull, null, null_mut};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::{
    function_graph_reduction_result::FunctionGraphReductionResult,
    type_function_context::TypeFunctionContext, type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};
impl TypeFunctionReducer {
  pub fn new(
    queued_tys: VecDeque<TypeId>,
    queued_tps: VecDeque<TypePackId>,
    should_guess: TypeOrTypePackIdSet,
    cyclic_types: Vec<TypeId>,
    location: Location,
    ctx: NonNull<TypeFunctionContext>,
    force: bool,
  ) -> Self {
    Self {
      ctx,
      queued_tys,
      queued_tps,
      should_guess,
      cyclic_type_functions: cyclic_types,
      irreducible: TypeOrTypePackIdSet::new(null_mut()),
      result: FunctionGraphReductionResult {
        errors: Vec::new(),
        messages: Vec::new(),
        blocked_types: DenseHashSet::new(null()),
        blocked_packs: DenseHashSet::new(null()),
        reduced_types: DenseHashSet::new(null()),
        reduced_packs: DenseHashSet::new(null()),
        irreducible_types: DenseHashSet::new(null()),
      },
      force,
      location,
    }
  }
}
