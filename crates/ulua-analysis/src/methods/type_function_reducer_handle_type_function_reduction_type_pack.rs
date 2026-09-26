//! C++ `TypeFunctionReducer::handleTypeFunctionReduction<T>` 的
//! `T = TypePackId` 单体（TypeFunction.cpp:375-453）。

use alloc::vec::Vec;
use core::mem::take;

use crate::{
  enums::{reduction::Reduction, type_function_instance_state::TypeFunctionInstanceState},
  records::{
    reduce_constraint::ReduceConstraint, type_error::TypeError,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
    user_defined_type_function_error::UserDefinedTypeFunctionError,
  },
  type_aliases::{
    constraint_v::ConstraintV, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl TypeFunctionReducer {
  /// # Safety
  /// 调用方须满足 C++ 原实现的调用契约（ctx 裸指针有效）。
  pub(crate) fn handle_type_function_reduction_type_pack_id(
    &mut self,
    subject: TypePackId,
    mut reduction: TypeFunctionReductionResult<TypePackId>,
  ) {
    for message in take(&mut reduction.messages) {
      self
        .result
        .messages
        .push(TypeError::type_error_location_type_error_data(
          self.location,
          TypeErrorData::UserDefinedTypeFunctionError(UserDefinedTypeFunctionError::new(message)),
        ));
    }

    if let Some(result_tp) = reduction.result {
      self.replace_type_pack_id(subject, result_tp);

      // 先取快照再入队，避免对 ctx 的借用横跨自有队列的可变操作（同 type 单体）。
      // self.ctx 是 reducer 构造期由真实 `&mut` 借用接线的 `Handle<TypeFunctionContext>`
      // （类型编码非空），活过整次 reduction；此处仅共享只读拷贝 fresh_instances
      // 与判空 solver，读取止于语句末，不与后续写冲突。
      let fresh: Vec<TypeId> = self.ctx.get().fresh_instances.clone();
      // 同一 Handle 不变量的又一次短读；solver 为可空裸句柄，只比较身份不解引用。
      let has_solver = !self.ctx.get().solver.is_null();
      for ty in fresh {
        self.queued_tys.push_back(ty);
        if has_solver {
          // C++: `ctx->pushConstraint(ReduceConstraint{ty})`.
          // ctx 存活（同上）；push_constraint 取 &self 只读再借用，
          // 与本循环对 self 的可变借用分属不同对象，无别名冲突。
          self
            .ctx
            .get()
            .push_constraint(ConstraintV::Reduce(ReduceConstraint { ty }));
        }
      }
    } else {
      self.irreducible.insert(subject as *const ());

      if let Some(error) = reduction.error.take() {
        self
          .result
          .errors
          .push(TypeError::type_error_location_type_error_data(
            self.location,
            TypeErrorData::UserDefinedTypeFunctionError(UserDefinedTypeFunctionError::new(error)),
          ));
      }

      if reduction.reduction_status != Reduction::MaybeOk || self.force {
        // getState(TypePackId) 恒为 Unsolved；setState 为 cpp 显式 no-op。
        if self.get_state_type_pack_id(subject) == TypeFunctionInstanceState::Unsolved {
          if reduction.reduction_status == Reduction::Erroneous {
            self.set_state_type_pack_id(subject, TypeFunctionInstanceState::Stuck);
          } else if reduction.reduction_status == Reduction::Irreducible {
            self.set_state_type_pack_id(subject, TypeFunctionInstanceState::Solved);
          } else if reduction.reduction_status == Reduction::MaybeOk {
            // We cannot make progress because something is unsolved, but we're also forcing.
            self.set_state_type_pack_id(subject, TypeFunctionInstanceState::Stuck);
          } else {
            // Safety: ice 是 ctx 构造期接线的 NonNull<InternalErrorReporter>，
            // as_ref 取只读借用非空有类型保证；ice_string 仅读 reporter 状态并
            // 走抛出路径，ctx 活过本分支。
            unsafe {
              self
                .ctx
                .get()
                .ice
                .as_ref()
                .ice_string("Unexpected TypeFunctionInstanceState");
            }
          }
        }

        // C++: `else if constexpr (std::is_same_v<T, TypePackId>)` — pack 分支
        // 不查 userFunc，直接记 UninhabitedTypePackFunction。
        self
          .result
          .errors
          .push(TypeError::type_error_location_type_error_data(
            self.location,
            TypeErrorData::UninhabitedTypePackFunction(UninhabitedTypePackFunction { tp: subject }),
          ));
      } else if reduction.reduction_status == Reduction::MaybeOk && !self.force {
        // We're not forcing and the reduction couldn't proceed, but it isn't obviously busted.
        // Report that this type blocks further reduction.
        for b in reduction.blocked_types.iter().copied() {
          self.result.blocked_types.insert(b);
        }

        for b in reduction.blocked_packs.iter().copied() {
          self.result.blocked_packs.insert(b);
        }
      } else {
        debug_assert!(false, "Unreachable");
      }
    }

    // ctx 为构造期接线、活过整次 reduction 的 Handle；函数尾此处
    // 只余这一条对 fresh_instances 的可变视图（上方快照已克隆、push 借用
    // 止于各自返回），单线程串行清空无并存别名。
    self.ctx.get_mut().fresh_instances.clear();
  }
}
