//! `reduceTypeFunctions(TypePackId entrypoint, ...)` (TypeFunction.cpp:722-747).

use alloc::{
  string::String,
  vec::{Vec, Vec as AllocVec},
};
use core::ptr::{NonNull, null};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  functions::{
    reduce_functions_internal::reduce_functions_internal,
    reduce_type_functions_type_function::is_recursion_limit_panic,
  },
  records::{
    function_graph_reduction_result::FunctionGraphReductionResult,
    generic_type_visitor::GenericTypeVisitorTrait, instance_collector::InstanceCollector,
    type_function_context::TypeFunctionContext, type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};
pub fn reduce_type_functions(
  entrypoint: TypePackId,
  location: Location,
  ctx: NonNull<TypeFunctionContext>,
  force: bool,
) -> FunctionGraphReductionResult {
  let mut collector = InstanceCollector {
    base: TypeOnceVisitor::new(String::from("InstanceCollector"), true),
    recorded_tys: DenseHashSet::new(TypeId::default()),
    tys: VecDeque::new(),
    recorded_tps: DenseHashSet::new(TypePackId::default()),
    tps: VecDeque::new(),
    should_guess: TypeOrTypePackIdSet::default(),
    type_function_instance_stack: Vec::new(),
    cyclic_instance: Vec::new(),
  };

  // C++ wraps this in `try { ... } catch (RecursionLimitException&) { return {}; }`.
  if let Err(payload) = catch_unwind(AssertUnwindSafe(|| {
    collector.traverse_type_pack_id(entrypoint)
  })) {
    if !is_recursion_limit_panic(&payload) {
      resume_unwind(payload);
    }

    return empty_reduction_result();
  }

  if collector.tys.empty() && collector.tps.empty() {
    return empty_reduction_result();
  }

  reduce_functions_internal(
    collector.tys,
    collector.tps,
    collector.should_guess,
    collector.cyclic_instance,
    location,
    ctx,
    force,
  )
}

fn empty_reduction_result() -> FunctionGraphReductionResult {
  FunctionGraphReductionResult {
    errors: AllocVec::new(),
    messages: AllocVec::new(),
    blocked_types: DenseHashSet::new(null()),
    blocked_packs: DenseHashSet::new(null()),
    reduced_types: DenseHashSet::new(null()),
    reduced_packs: DenseHashSet::new(null()),
    irreducible_types: DenseHashSet::new(null()),
  }
}
