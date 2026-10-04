//! `type_function_reducer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault, vec_deque::VecDeque},
};

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{follow_type_pack, get_mutable_type, get_type, get_type_pack},
  records::{
    arena_handle::{Handle, alias_ref},
    function_graph_reduction_result::FunctionGraphReductionResult,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
    visit_key::VisitKeyRef,
  },
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};

impl TypeFunctionReducer {
  pub fn done(&self) -> bool {
    self.queued_tys.empty() && self.queued_tps.empty()
  }
}

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:346 getState(TypeId)`。
  pub fn get_state_type_id(&self, ty: TypeId) -> TypeFunctionInstanceState {
    let tfit = get_type::get::<TypeFunctionInstanceType>(ty);
    LUAU_ASSERT!(tfit.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some，与 C++ `LUAU_ASSERT(tfit); tfit->state` 一致
    tfit.expect("紧邻 LUAU_ASSERT(tfit.is_some()) 蕴含").state
  }
}

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:365 getState(TypePackId)`。
  pub fn get_state_type_pack_id(&self, _tp: TypePackId) -> TypeFunctionInstanceState {
    TypeFunctionInstanceState::Unsolved
  }
}

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:353 setState(TypeId, ...)`。
  /// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub(crate) fn set_state_type_id_type_function_instance_state(
    &self,
    ty: TypeId,
    state: TypeFunctionInstanceState,
  ) {
    if alias_ref(ty).owning_arena != self.ctx.get().arena_id() {
      return;
    }

    let tfit = get_mutable_type::get_mutable::<TypeFunctionInstanceType>(ty);
    LUAU_ASSERT!(tfit.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some，与 C++ `LUAU_ASSERT(tfit); tfit->state = state` 一致
    tfit.expect("紧邻 LUAU_ASSERT(tfit.is_some()) 蕴含").state = state;
  }
}

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:370 setState(TypePackId, ...)`：上游即为显式 no-op
  /// （"We do not presently have any type pack functions at all."），保留同名
  /// 单体以对齐调用点；出现带状态的 pack family 时在此落地。
  pub fn set_state_type_pack_id(&self, _tp: TypePackId, _state: TypeFunctionInstanceState) {}
}

impl TypeFunctionReducer {
  pub fn step(&mut self) {
    if !self.queued_tys.empty() {
      self.step_type();
    } else if !self.queued_tps.empty() {
      self.step_pack();
    }
  }
}

// `TypeFunctionReducer::stepPack` (TypeFunction.cpp:624-646).

impl TypeFunctionReducer {
  pub fn step_pack(&mut self) {
    // SAFETY: queued_tps 内的句柄由构造方按 C++ 契约保证有效（同 stepType 的 follow）。
    let subject = follow_type_pack::follow(*self.queued_tps.front());
    self.queued_tps.pop_front();

    if self.irreducible.contains(&VisitKeyRef::from_ptr(subject)) {
      return;
    }

    if let Some(tfit) = get_type_pack::get::<TypeFunctionInstanceTypePack>(subject) {
      if !self.test_parameters_type_pack_id(subject, tfit) {
        return;
      }

      if self.try_guessing(subject) {
        return;
      }

      // C++: `tfit->function->reducer(subject, tfit->typeArguments, tfit->packArguments, ctx)`
      // reducer 为 safe `fn` 指针（契约见 `ReducerFunction` 文档的调用序段）；
      // `get_mut()` 从构造期接线的 Handle（非空由类型编码，构造点为真实 `&mut`
      // 借用）物化本次调用的独占会话借用（上下文为驱动栈帧局物，存活覆盖整条
      // 归约队列），句柄有效性与 C++ 同契约。
      let reducer = tfit.function().reducer;
      let result: TypeFunctionReductionResult<_> = reducer(
        subject,
        &tfit.type_arguments,
        &tfit.pack_arguments,
        self.ctx.get_mut(),
      );
      self.handle_type_function_reduction_type_pack_id(subject, result);
    }
  }
}

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
      irreducible: TypeOrTypePackIdSet::new(VisitKeyRef::dense_default()),
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
