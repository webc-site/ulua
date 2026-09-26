//! `TypeFunctionReducer::handleTypeFunctionReduction<T>` (TypeFunction.cpp:375-453).
//!
//! C++ is a template branching on `std::is_same_v<T, TypeId|TypePackId>`. It is
//! rendered as the two concrete `handle_type_function_reduction_type_id` (本文件)
//! 与 `..._type_pack_id`（`..._reduction_type_pack.rs`）单体；结果结构体
//! `TypeFunctionReductionResult<T = TypeId>` 与之同参。

use alloc::vec::Vec;
use core::{mem::take, ptr::eq};

use crate::{
  enums::{reduction::Reduction, type_function_instance_state::TypeFunctionInstanceState},
  functions::get_type,
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
      // `self.ctx` 对应 C++ `TypeFunctionContext& ctx`——由
      // `reduce_functions_internal` 调用链在构造期以真实 `&mut` 借用注入的存活
      // 上下文（`Handle` 类型编码非空），生命周期覆盖整个消解；此处仅克隆
      // fresh_instances（只读），并刻意先取出以免跨后续对 self 的可变借用持有 ctx 借用。
      let fresh: Vec<TypeId> = self.ctx.get().fresh_instances.clone();
      // 同上，ctx 存活；仅读取 solver 裸指针做判空比较，不解引用。
      let has_solver = !self.ctx.get().solver.is_null();
      for ty in fresh {
        self.queued_tys.push_back(ty);
        if has_solver {
          // C++: `ctx->pushConstraint(ReduceConstraint{ty})`.
          // ctx 存活（同上）；has_solver 刚刚以同一指针链确认非空，
          // push_constraint 对 `&self` 操作，与 `ty`（本循环按值拷出的句柄）
          // 无别名交叉。
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
        if self.get_state_type_id(subject) == TypeFunctionInstanceState::Unsolved {
          if reduction.reduction_status == Reduction::Erroneous {
            // Safety: `subject` 是本轮消解队列里的存活 TypeId（上方
            // get_state_type_id 已按其可解引用），且已确认为
            // TypeFunctionInstanceType 变体；ctx 存活（函数头论证）——满足
            // set_state 的 C++ setState(TypeId,…) 契约。
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Stuck,
              )
            };
          } else if reduction.reduction_status == Reduction::Irreducible {
            // Safety: 同 Stuck 分支——subject 有效性刚由 get_state_type_id
            // 验证，状态迁移 Irreducible→Solved 与 C++ 分支一一对应。
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Solved,
              )
            };
          } else if reduction.reduction_status == Reduction::MaybeOk {
            // We cannot make progress because something is unsolved, but we're also forcing.
            // Safety: 同上——subject 为刚验证的存活实例句柄，MaybeOk+force
            // 走 Stuck 与 C++ 语义一致。
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Stuck,
              )
            };
          } else {
            // Safety: ctx 存活且 ice 是构造期注入的非空
            // NotNull<InternalErrorReporter>；as_ref 只读借用止于本调用，
            // 单线程下无并发访问。
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

        // C++: `if (const TypeFunctionInstanceType* tf = get<...>(subject))`
        if let Some(tf) = get_type::get::<TypeFunctionInstanceType>(subject) {
          // C++: `tf->function != &ctx->builtins->typeFunctions->userFunc`
          // Safety: ctx 存活（函数头论证）；builtins 为构造期注入的非空
          // NotNull<BuiltinTypes> 单例，user_func 是其内嵌注册表的固定地址；
          // 两侧仅取地址比较指针（同 C++ 引用比较），不解引用 tf 之外的内容。
          let is_user_func = unsafe {
            let ctx = self.ctx.get();
            eq(
              tf.function.as_ptr() as *const _,
              &ctx.builtins.as_ref().type_functions.user_func as *const _,
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

    // ctx 由调用链持有且存活至 reducer 析构（函数头论证）；清空的是
    // ctx 自己的 freshInstances（cpp:452），此刻对它的只读迭代（上方 fresh
    // 循环）已结束，无借用重叠。
    self.ctx.get_mut().fresh_instances.clear();
  }
}
