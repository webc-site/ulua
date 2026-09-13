//! `TypeFunctionReducer::handleTypeFunctionReduction<T>` (TypeFunction.cpp:375-453).
//!
//! C++ is a template branching on `std::is_same_v<T, TypeId|TypePackId>`. It is
//! rendered as the two concrete `handle_type_function_reduction_type_id` /
//! `..._type_pack_id` methods. The reduction result struct in this crate is
//! monomorphized on `TypeId`.

use alloc::vec::Vec;
use core::{ffi::c_void, mem::take, ptr::eq};

use crate::{
  enums::{reduction::Reduction, type_function_instance_state::TypeFunctionInstanceState},
  functions::get_type_alt_j::get_type_id,
  records::{
    reduce_constraint::ReduceConstraint, type_error::TypeError,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
    uninhabited_type_function::UninhabitedTypeFunction,
    user_defined_type_function_error::UserDefinedTypeFunctionError,
  },
  type_aliases::{constraint_v::ConstraintV, type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeFunctionReducer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn handle_type_function_reduction_type_id(
    &mut self,
    subject: TypeId,
    mut reduction: TypeFunctionReductionResult,
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

    if let Some(result_ty) = reduction.result {
      self.replace_type_id(subject, result_ty);

      // Collect fresh instances first so we are not holding the borrow on
      // ctx across the mutable push onto our own queue.
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
        if self.get_state_type_id(subject) == TypeFunctionInstanceState::Unsolved {
          if reduction.reduction_status == Reduction::Erroneous {
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Stuck,
              )
            };
          } else if reduction.reduction_status == Reduction::Irreducible {
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Solved,
              )
            };
          } else if reduction.reduction_status == Reduction::MaybeOk {
            // We cannot make progress because something is unsolved, but we're also forcing.
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Stuck,
              )
            };
          } else {
            unsafe {
              (*self.ctx.as_ptr())
                .ice
                .as_ref()
                .ice_string("Unexpected TypeFunctionInstanceState");
            }
          }
        }

        // C++: `if (const TypeFunctionInstanceType* tf = get<...>(subject))`
        if let Some(tf) = get_type_id::<TypeFunctionInstanceType>(subject) {
          // C++: `tf->function != &ctx->builtins->typeFunctions->userFunc`
          // SAFETY: ctx 由 reducer 构造方保证存活。
          let is_user_func = unsafe {
            eq(
              tf.function.as_ptr() as *const _,
              &(*self.ctx.as_ptr())
                .builtins
                .as_ref()
                .type_functions
                .user_func as *const _,
            )
          };
          if !is_user_func {
            self
              .result
              .errors
              .push(TypeError::type_error_location_type_error_data(
                self.location,
                TypeErrorData::UninhabitedTypeFunction(UninhabitedTypeFunction { ty: subject }),
              ));
          }
        }
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
