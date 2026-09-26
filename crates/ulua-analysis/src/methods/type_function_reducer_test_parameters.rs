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
  /// C++ `TypeFunctionReducer::testParameters<T>` 的 `T = TypeId` 单体。
  pub(crate) fn test_parameters_type_id(
    &mut self,
    subject: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    for p in &tfit.type_arguments {
      let skip = self.test_for_skippability_type_id(*p);

      if skip == SkipTestResult::Stuck {
        self.irreducible.insert(subject as *const ());
        // Safety: set_state_* 为 unsafe fn，其前置「ty 为存活 TypeId」由 subject
        // （本 reducer 正在归约的 arena 节点句柄，bump 分配地址稳定）满足；
        // 函数内部经 ctx 的 NonNull 字段比对 owning_arena 后短借 map，无并存别名。
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
          // Safety: function 是 TypeFunctionInstanceType 构造时以 NonNull 登记的
          // 类型函数定义句柄（builtin/arena 持有，会话期内存活），as_ptr 解引用
          // 仅按值读 can_reduce_generics: bool。
          && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const ());

        if skip == SkipTestResult::Generic {
          // Safety: 同 Stuck 分支——subject 为本归约期存活 arena 句柄，
          // set_state_* 内部借用窗口随返回结束。
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
          // Safety: 与 type 侧同一不变量——tfit.function 为构造期 NonNull 登记、
          // 会话期存活的类型函数定义，此处仅按值读 can_reduce_generics。
          && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const ());
        return false;
      } else if skip == SkipTestResult::Defer {
        self.queued_tys.push_back(subject);
        return false;
      }
    }

    true
  }
}
