//! `TypeFunctionReducer::stepType` (TypeFunction.cpp:565-622).

use crate::{
  enums::{
    skip_test_result::SkipTestResult, type_function_instance_state::TypeFunctionInstanceState,
  },
  functions::{follow_type::follow, get_type},
  records::{
    generic_type_visitor::GenericTypeVisitorTrait,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
    unscoped_generic_finder::UnscopedGenericFinder,
  },
};
impl TypeFunctionReducer {
  pub fn step_type(&mut self) {
    let subject = follow(*self.queued_tys.front());
    self.queued_tys.pop_front();

    if self.irreducible.contains(&(subject as *const ())) {
      return;
    }

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(subject) {
      // tfit->function->name == "user"
      // SAFETY: function 指向内建类型函数表项，存活期同 arena。
      let is_user = unsafe { tfit.function.as_ref().name == "user" };
      if is_user {
        let mut finder = UnscopedGenericFinder::new();
        finder.traverse_type_id(subject);

        if finder.found_unscoped {
          // Do not step into this type again
          self.irreducible.insert(subject as *const ());

          // Let the caller know this type will not become reducible
          self.result.irreducible_types.insert(subject);

          if self.get_state_type_id(subject) == TypeFunctionInstanceState::Unsolved {
            unsafe {
              self.set_state_type_id_type_function_instance_state(
                subject,
                TypeFunctionInstanceState::Solved,
              )
            };
          }

          return;
        }
      }

      let test_cyclic = self.test_for_skippability_type_id(subject);

      if !self.test_parameters_type_id(subject, tfit)
        && test_cyclic != SkipTestResult::CyclicTypeFunction
      {
        let state = tfit.state;
        if matches!(
          state,
          TypeFunctionInstanceState::Stuck | TypeFunctionInstanceState::Solved
        ) {
          self.try_guessing(subject);
        }

        return;
      }

      if self.try_guessing(subject) {
        return;
      }

      // ctx 由 reducer 构造方以存活 `&mut` 借用接线（Handle 编码非空），
      // 这里是 cpp `ctx->userFuncName = ...` 的直译写回。
      self.ctx.get_mut().user_func_name = tfit.user_func_name;

      // C++: `tfit->function->reducer(subject, tfit->typeArguments, tfit->packArguments, ctx)`
      // SAFETY: reducer 为内建函数指针；`get_mut()` 从构造期接线的 Handle 物化本次
      // 调用的独占会话借用（上下文为驱动栈帧局物，存活覆盖整条归约队列），
      // 句柄有效性与 C++ 同契约。
      let result: TypeFunctionReductionResult = unsafe {
        let reducer = tfit.function.as_ref().reducer;
        reducer(
          subject,
          &tfit.type_arguments,
          &tfit.pack_arguments,
          self.ctx.get_mut(),
        )
      };
      self.handle_type_function_reduction_type_id(subject, result);
    }
  }
}
