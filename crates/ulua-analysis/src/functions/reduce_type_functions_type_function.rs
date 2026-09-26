use alloc::{
  string::String,
  vec::{Vec, Vec as AllocVec},
};
use core::any::Any;
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
  type Seen = DenseHashSet<*mut ()>;

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
  ctx: &mut TypeFunctionContext,
  force: bool,
) -> FunctionGraphReductionResult {
  let mut collector = InstanceCollector {
    base: TypeOnceVisitor::new(String::from("InstanceCollector"), true),
    recorded_tys: DenseHashSet::default(),
    tys: VecDeque::new(),
    recorded_tps: DenseHashSet::default(),
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
    blocked_types: DenseHashSet::default(),
    blocked_packs: DenseHashSet::default(),
    reduced_types: DenseHashSet::default(),
    reduced_packs: DenseHashSet::default(),
    irreducible_types: DenseHashSet::default(),
  }
}

/// 判定 panic 载荷是否为递归上限溢出（C++ `catch (RecursionLimitException&)`）。
/// `dyn Any` 保留：形参类型即 `catch_unwind` 的错误形态，载荷类型集合由
/// panic 运行期擦除，编译期不可枚举。
pub(crate) fn is_recursion_limit_panic(payload: &(dyn Any + Send)) -> bool {
  const PREFIX: &str = "Internal recursion counter limit exceeded";

  if let Some(message) = payload.downcast_ref::<&str>() {
    message.starts_with(PREFIX)
  } else if let Some(message) = payload.downcast_ref::<String>() {
    message.starts_with(PREFIX)
  } else {
    false
  }
}

pub fn reduce_type_functions_type_pack_id(
  entrypoint: TypePackId,
  location: Location,
  ctx: &mut TypeFunctionContext,
  force: bool,
) -> FunctionGraphReductionResult {
  let mut collector = InstanceCollector {
    base: TypeOnceVisitor::new(String::from("InstanceCollector"), true),
    recorded_tys: DenseHashSet::default(),
    tys: VecDeque::new(),
    recorded_tps: DenseHashSet::default(),
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
