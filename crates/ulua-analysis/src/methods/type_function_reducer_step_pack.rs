//! `TypeFunctionReducer::stepPack` (TypeFunction.cpp:624-646).

use crate::{
  functions::{follow_type_pack, get_type_pack},
  records::{
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
};

impl TypeFunctionReducer {
  pub fn step_pack(&mut self) {
    // SAFETY: queued_tps 内的句柄由构造方按 C++ 契约保证有效（同 stepType 的 follow）。
    let subject = follow_type_pack::follow(*self.queued_tps.front());
    self.queued_tps.pop_front();

    if self.irreducible.contains(&(subject as *const ())) {
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
      // SAFETY: reducer 为函数指针；`get_mut()` 从构造期接线的 Handle（非空由类型
      // 编码，构造点为真实 `&mut` 借用）物化本次调用的独占会话借用（上下文为驱动
      // 栈帧局物，存活覆盖整条归约队列），句柄有效性与 C++ 同契约。
      let result: TypeFunctionReductionResult<_> = unsafe {
        let reducer = (*tfit.function).reducer;
        reducer(
          subject,
          &tfit.type_arguments,
          &tfit.pack_arguments,
          self.ctx.get_mut(),
        )
      };
      self.handle_type_function_reduction_type_pack_id(subject, result);
    }
  }
}
