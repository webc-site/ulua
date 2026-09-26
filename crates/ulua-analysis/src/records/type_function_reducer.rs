use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::vec_deque::VecDeque;

use crate::{
  records::{
    arena_handle::Handle, function_graph_reduction_result::FunctionGraphReductionResult,
    type_function_context::TypeFunctionContext,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};

/// cpp `TypeFunctionReducer`（`Analysis/src/TypeFunction.cpp:199-201`）的首成员
/// `NotNull<TypeFunctionContext> ctx`：整条归约队列所别名的会话上下文。
///
/// # 为何是 [`Handle`] 而不是 `&'a mut TypeFunctionContext`
/// reducer 的寿命比一次借用长、且方法体刻意让 ctx 读写与自身队列写交错推进
/// （cpp 同一 `ctx->` 直解形态）：`try_guessing` 先读 `ctx.arena/builtins/normalizer`
/// 造 guesser，再回头以 `&mut self` 调 `replace_*`；`handle_type_function_reduction`
/// 从 `ctx.fresh_instances` 取快照后立即 `self.queued_tys.push_back`；`step_*` 又把
/// `ctx` 独占借给 `ReducerFunction`。这些都要求「同一时刻经 ctx 的借用不与对 self 的
/// 借用互相约束」，正是原 `NonNull` 裸解引用的借用检查行为；换成 `&'a mut` 字段会迫使
/// 逐个重排控制流（把快照/复校拆成两轮），那是发明 oracle 里没有的顺序。故沿用
/// [`Handle`]：非空由类型编码，构造点收敛为真实 `&mut` 借用（见
/// `TypeFunctionReducer::new`），解引用收进 `arena_handle` 一处。
#[derive(Debug, Clone)]
pub struct TypeFunctionReducer {
  pub ctx: Handle<TypeFunctionContext>,
  pub queued_tys: VecDeque<TypeId>,
  pub queued_tps: VecDeque<TypePackId>,
  pub should_guess: TypeOrTypePackIdSet,
  pub cyclic_type_functions: Vec<TypeId>,
  pub irreducible: TypeOrTypePackIdSet,
  pub result: FunctionGraphReductionResult,
  pub force: bool,
  pub location: Location,
}
