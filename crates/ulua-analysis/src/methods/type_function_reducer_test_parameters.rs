use core::ffi::c_void;

use crate::{
  enums::{
    skip_test_result::SkipTestResult, type_function_instance_state::TypeFunctionInstanceState,
  },
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReducer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn test_parameters(
    &mut self,
    subject: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    for p in &tfit.type_arguments {
      let skip = self.test_for_skippability_type_id(*p);

      if skip == SkipTestResult::Stuck {
        self.irreducible.insert(subject as *const c_void);
        unsafe {
          self.set_state_type_id_type_function_instance_state(
            subject,
            TypeFunctionInstanceState::Stuck,
          )
        };

        return false;
      }

      if skip == SkipTestResult::Irreducible
        || (skip == SkipTestResult::Generic
          && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const c_void);

        if skip == SkipTestResult::Generic {
          unsafe {
            self.set_state_type_id_type_function_instance_state(
              subject,
              TypeFunctionInstanceState::Solved,
            )
          };
        }

        return false;
      } else if skip == SkipTestResult::Defer {
        self.queued_tys.push_back(subject);
        return false;
      }
    }

    for p in &tfit.pack_arguments {
      let skip = self.test_for_skippability_type_pack_id(*p);

      if skip == SkipTestResult::Irreducible
        || (skip == SkipTestResult::Generic
          && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const c_void);
        return false;
      } else if skip == SkipTestResult::Defer {
        self.queued_tys.push_back(subject);
        return false;
      }
    }

    true
  }
}
