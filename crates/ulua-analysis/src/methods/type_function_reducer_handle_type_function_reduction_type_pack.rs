//! C++ `TypeFunctionReducer::handleTypeFunctionReduction<T>` 的
//! `T = TypePackId` 单体（TypeFunction.cpp:375-453）。

use alloc::vec::Vec;
use core::{ffi::c_void, mem::take};

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
      let fresh: Vec<TypeId> = unsafe { (*self.ctx.as_ptr()).fresh_instances.clone() };
      let has_solver = unsafe { !(*self.ctx.as_ptr()).solver.is_null() };
      for ty in fresh {
        self.queued_tys.push_back(ty);
        if has_solver {
          // C++: `ctx->pushConstraint(ReduceConstraint{ty})`.
          unsafe {
            (*self.ctx.as_ptr()).push_constraint(ConstraintV::Reduce(ReduceConstraint { ty }));
          }
        }
      }
    } else {
      self.irreducible.insert(subject as *const c_void);

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
            unsafe {
              (*self.ctx.as_ptr())
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

    unsafe {
      (*self.ctx.as_ptr()).fresh_instances.clear();
    }
  }
}
