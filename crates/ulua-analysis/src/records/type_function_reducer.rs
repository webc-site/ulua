use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::records::vec_deque::VecDeque;

use crate::{
  records::{
    function_graph_reduction_result::FunctionGraphReductionResult,
    type_function_context::TypeFunctionContext,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};

#[derive(Debug, Clone)]
pub struct TypeFunctionReducer {
  pub ctx: NonNull<TypeFunctionContext>,
  pub queued_tys: VecDeque<TypeId>,
  pub queued_tps: VecDeque<TypePackId>,
  pub should_guess: TypeOrTypePackIdSet,
  pub cyclic_type_functions: Vec<TypeId>,
  pub irreducible: TypeOrTypePackIdSet,
  pub result: FunctionGraphReductionResult,
  pub force: bool,
  pub location: Location,
}
