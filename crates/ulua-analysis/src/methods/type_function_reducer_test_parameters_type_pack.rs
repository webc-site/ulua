use crate::{
  enums::{
    skip_test_result::SkipTestResult, type_function_instance_state::TypeFunctionInstanceState,
  },
  records::{
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeFunctionReducer {
  /// C++ `TypeFunctionReducer::testParameters<T>` 的 `T = TypePackId` 单体。
  /// Defer 分支回投 `queued_tps`（cpp `if constexpr` 选择）；`setState(TypePackId)`
  /// 在 cpp 里就是显式 no-op（"We do not presently have any type pack functions"），
  /// 故此处只记 irreducible 不记状态。
  pub(crate) fn test_parameters_type_pack_id(
    &mut self,
    subject: TypePackId,
    tfit: &TypeFunctionInstanceTypePack,
  ) -> bool {
    for p in &tfit.type_arguments {
      let skip = self.test_for_skippability_type_id(*p);

      if skip == SkipTestResult::Stuck {
        self.irreducible.insert(subject as *const ());
        self.set_state_type_pack_id(subject, TypeFunctionInstanceState::Stuck);

        return false;
      }

      if skip == SkipTestResult::Irreducible
        || (skip == SkipTestResult::Generic
          // Safety: `tfit.function` 是 TypeFunctionInstanceTypePack 构造时登记的
          // 类型 pack 函数定义句柄（builtin 函数表持有、会话期内存活、非空对齐），
          // 与 type 侧 test_parameters 同一不变量；此处仅按值读 `can_reduce_generics: bool`。
          && unsafe { !(*tfit.function).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const ());

        if skip == SkipTestResult::Generic {
          self.set_state_type_pack_id(subject, TypeFunctionInstanceState::Solved);
        }

        return false;
      } else if skip == SkipTestResult::Defer {
        self.queued_tps.push_back(subject);
        return false;
      }
    }

    for p in &tfit.pack_arguments {
      let skip = self.test_for_skippability_type_pack_id(*p);

      if skip == SkipTestResult::Irreducible
        || (skip == SkipTestResult::Generic
          // Safety: 同 pack 循环首段——`tfit.function` 为构造期登记、会话期存活的
          // 类型 pack 函数定义，此处仅按值读 `can_reduce_generics`，只读无别名。
          && unsafe { !(*tfit.function).can_reduce_generics })
      {
        self.irreducible.insert(subject as *const ());
        return false;
      } else if skip == SkipTestResult::Defer {
        self.queued_tps.push_back(subject);
        return false;
      }
    }

    true
  }
}
