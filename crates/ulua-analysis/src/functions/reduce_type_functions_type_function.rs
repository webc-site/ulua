//! `reduceTypeFunctions(TypeId entrypoint, ...)` (TypeFunction.cpp:695-720).

/// Wires `InstanceCollector`'s virtual surface (TypeFunction.cpp:41-135) into
/// the `GenericTypeVisitor` traversal machinery so that `traverse` dispatches
/// into its overrides — the same pattern as `FindCyclicTypes` /
/// `InternalTypeFunctionFinder`. The per-`visit`/`cycle` bodies live on the
/// `InstanceCollector` record's inherent methods; this impl delegates to them.
use alloc::vec::Vec as AllocVec;
use alloc::{string::String, vec::Vec};
use core::{
  any::Any,
  ffi::c_void,
  ptr::{NonNull, null},
};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  functions::reduce_functions_internal::reduce_functions_internal,
  records::{
    extern_type::ExternType,
    function_graph_reduction_result::FunctionGraphReductionResult,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    instance_collector::InstanceCollector,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};
impl GenericTypeVisitorTrait for InstanceCollector {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn cycle_type_id(&mut self, ty: TypeId) {
    InstanceCollector::cycle(self, ty);
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    InstanceCollector::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    InstanceCollector::visit_type_id_extern_type(self, ty, etv)
  }

  fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    InstanceCollector::visit_type_pack_id_type_function_instance_type_pack(self, tp, tfitp)
  }
}

pub fn reduce_type_functions(
  entrypoint: TypeId,
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
  if let Err(payload) = catch_unwind(AssertUnwindSafe(|| collector.traverse_type_id(entrypoint))) {
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

fn is_recursion_limit_panic(payload: &Box<dyn Any + Send>) -> bool {
  const PREFIX: &str = "Internal recursion counter limit exceeded";

  if let Some(message) = payload.downcast_ref::<&str>() {
    message.starts_with(PREFIX)
  } else if let Some(message) = payload.downcast_ref::<String>() {
    message.starts_with(PREFIX)
  } else {
    false
  }
}
