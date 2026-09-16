//! `reduceFunctionsInternal` (TypeFunction.cpp:657-693).

use alloc::vec::Vec;
use core::{
  mem::replace,
  ptr::{NonNull, null},
};

use ulua_ast::records::location::Location;
use ulua_common::{
  DFInt,
  records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque},
};

use crate::{
  records::{
    code_too_complex::CodeTooComplex,
    function_graph_reduction_result::FunctionGraphReductionResult, type_error::TypeError,
    type_function_context::TypeFunctionContext, type_function_reducer::TypeFunctionReducer,
    type_reduction_reentrancy_guard::TypeReductionReentrancyGuard,
  },
  type_aliases::{
    type_error_data::TypeErrorData, type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet,
    type_pack_id::TypePackId,
  },
};
pub fn reduce_functions_internal(
  queued_tys: VecDeque<TypeId>,
  queued_tps: VecDeque<TypePackId>,
  should_guess: TypeOrTypePackIdSet,
  cyclics: Vec<TypeId>,
  location: Location,
  ctx: NonNull<TypeFunctionContext>,
  force: bool,
) -> FunctionGraphReductionResult {
  let mut reducer = TypeFunctionReducer::new(
    queued_tys,
    queued_tps,
    should_guess,
    cyclics,
    location,
    ctx,
    force,
  );
  let mut iteration_count: i32 = 0;

  // If we are reducing a type function while reducing a type function,
  // we're probably doing something clowny. One known place this can
  // occur is type function reduction => overload selection => subtyping
  // => back to type function reduction. At worst, if there's a reduction
  // that _doesn't_ loop forever and _needs_ reentrancy, we'll fail to
  // handle that and potentially emit an error when we didn't need to.
  let shared_state = unsafe { (*(*ctx.as_ptr()).normalizer.as_ptr()).shared_state };
  if unsafe { !shared_state.is_null() && (*shared_state).reentrant_type_reduction } {
    return FunctionGraphReductionResult {
      errors: Vec::new(),
      messages: Vec::new(),
      blocked_types: DenseHashSet::new(null()),
      blocked_packs: DenseHashSet::new(null()),
      reduced_types: DenseHashSet::new(null()),
      reduced_packs: DenseHashSet::new(null()),
      irreducible_types: DenseHashSet::new(null()),
    };
  }

  // TypeReductionReentrancyGuard _{ctx->normalizer->sharedState};
  // RAII: sets reentrant_type_reduction = true now, resets to false on scope exit.
  let _guard = unsafe {
    TypeReductionReentrancyGuard::type_reduction_reentrancy_guard_not_null_unifier_shared_state(
      shared_state,
    )
  };

  let max_steps = DFInt::LuauTypeFamilyGraphReductionMaximumSteps.get();

  while !reducer.done() {
    reducer.step();

    iteration_count += 1;
    if iteration_count > max_steps {
      reducer
        .result
        .errors
        .push(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::CodeTooComplex(CodeTooComplex::default()),
        ));
      break;
    }
  }

  // The Rust `TypeReductionReentrancyGuard` has no `Drop` impl yet, so mirror
  // the C++ destructor (`sharedState->reentrantTypeReduction = false`) here at
  // scope exit to preserve the RAII semantics faithfully.
  unsafe {
    if !shared_state.is_null() {
      (*shared_state).reentrant_type_reduction = false;
    }
  }

  replace(
    &mut reducer.result,
    FunctionGraphReductionResult {
      errors: Vec::new(),
      messages: Vec::new(),
      blocked_types: DenseHashSet::new(null()),
      blocked_packs: DenseHashSet::new(null()),
      reduced_types: DenseHashSet::new(null()),
      reduced_packs: DenseHashSet::new(null()),
      irreducible_types: DenseHashSet::new(null()),
    },
  )
}
