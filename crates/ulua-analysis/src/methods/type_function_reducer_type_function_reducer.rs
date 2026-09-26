use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::{
    arena_handle::Handle, function_graph_reduction_result::FunctionGraphReductionResult,
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
    ctx: &mut TypeFunctionContext,
    force: bool,
  ) -> Self {
    Self {
      // cpp `NotNull<TypeFunctionContext> ctx{...}`：由调用帧的独占会话借用物化句柄，
      // 非空由类型编码，无需判空或 unchecked 裸构造。
      ctx: Handle::from_mut(ctx),
      queued_tys,
      queued_tps,
      should_guess,
      cyclic_type_functions: cyclic_types,
      irreducible: TypeOrTypePackIdSet::new(null_mut()),
      result: FunctionGraphReductionResult {
        errors: Vec::new(),
        messages: Vec::new(),
        blocked_types: DenseHashSet::default(),
        blocked_packs: DenseHashSet::default(),
        reduced_types: DenseHashSet::default(),
        reduced_packs: DenseHashSet::default(),
        irreducible_types: DenseHashSet::default(),
      },
      force,
      location,
    }
  }
}
