//! `TypeFunctionReducer::stepPack` (TypeFunction.cpp:624-646).

use core::ffi::c_void;

use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
};

impl TypeFunctionReducer {
  pub fn step_pack(&mut self) {
    // SAFETY: queued_tps 内的句柄由构造方按 C++ 契约保证有效（同 stepType 的 follow）。
    let subject = unsafe { follow_type_pack_id(*self.queued_tps.front()) };
    self.queued_tps.pop_front();

    if self.irreducible.contains(&(subject as *const c_void)) {
      return;
    }

    if let Some(tfit) = get_type_pack_id::<TypeFunctionInstanceTypePack>(subject) {
      if !self.test_parameters_type_pack_id(subject, tfit) {
        return;
      }

      if self.try_guessing(subject) {
        return;
      }

      // C++: `tfit->function->reducer(subject, tfit->typeArguments, tfit->packArguments, ctx)`
      // SAFETY: reducer 为函数指针，句柄有效性与 C++ 同契约。
      let result: TypeFunctionReductionResult<_> = unsafe {
        let reducer = (*tfit.function).reducer;
        reducer(
          subject,
          &tfit.type_arguments,
          &tfit.pack_arguments,
          self.ctx.as_ptr(),
        )
      };
      self.handle_type_function_reduction_type_pack_id(subject, result);
    }
  }
}
