use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
    instantiate::instantiate,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, free_type::FreeType,
    free_type_pack::FreeTypePack, function_type::FunctionType,
    push_function_type_constraint::PushFunctionTypeConstraint,
  },
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `constraint` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn try_dispatch_push_function_type_constraint_not_null_constraint(
    &mut self,
    c: &PushFunctionTypeConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let Some(mut expected_fn) =
      get_type_id::<FunctionType>(follow_type_id(c.expected_function_type))
    else {
      return true;
    };
    let Some(fn_ty) = get_type_id::<FunctionType>(follow_type_id(c.function_type)) else {
      // 若期望类型或给定类型不是函数，直接 bail。
      return true;
    };

    // cpp: `instantiate(builtinTypes, arena, NotNull{&limits}, constraint->scope, c.expectedFunctionType)`
    let instantiated = instantiate(
      // SAFETY: builtin_types/arena 在 solver 存活期内有效。
      unsafe { &*self.builtin_types },
      unsafe { &mut *self.arena },
      &self.limits,
      // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
      unsafe { (*constraint).scope },
      c.expected_function_type,
    );
    let Some(instantiated) = instantiated else {
      // cpp: 实例化失败，直接 bail。
      return true;
    };
    // cpp: `LUAU_ASSERT(expectedFn)` — 实例化结果必仍是函数类型。
    let new_expected_fn = get_type_id::<FunctionType>(follow_type_id(instantiated));
    LUAU_ASSERT!(new_expected_fn.is_some());
    if let Some(t) = new_expected_fn {
      expected_fn = t;
    }

    let mut expected_params = begin_type_pack_id(expected_fn.arg_types);
    let mut params = begin_type_pack_id(fn_ty.arg_types);

    let expected_params_end = end_type_pack_id(expected_fn.arg_types);
    let params_end = end_type_pack_id(fn_ty.arg_types);

    if expected_params.operator_eq(&expected_params_end) || params.operator_eq(&params_end) {
      return true;
    }

    if c.is_self {
      let params_current = *params.operator_deref();
      if get_type_id::<FreeType>(follow_type_id(params_current)).is_some() {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(constraint, params_current, {
            *expected_params.operator_deref()
          })
        };
      }
      expected_params.operator_inc();
      params.operator_inc();
    }

    let args = unsafe { (*c.expr).args.as_slice() };
    // `idx` 是附加 `AstExprFunction` 参数的索引；存在 `self` 时无需对参数偏移。
    for arg in args {
      if expected_params.operator_eq(&expected_params_end) || params.operator_eq(&params_end) {
        break;
      }

      let arg = *arg;
      let annotation = unsafe { (*arg).annotation };
      let params_current = *params.operator_deref();

      // 注解优先于一切，见到注解就 bail；非自由类型同样不在推断范围内。
      if annotation.is_null() && get_type_id::<FreeType>(follow_type_id(params_current)).is_some() {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(constraint, params_current, {
            *expected_params.operator_deref()
          })
        };
      }

      expected_params.operator_inc();
      params.operator_inc();
    }

    if unsafe { (*c.expr).return_annotation.is_null() }
      && get_type_pack_id::<FreeTypePack>(fn_ty.ret_types).is_some()
    {
      unsafe {
        self.bind_not_null_constraint_type_pack_id_type_pack_id(
          constraint,
          fn_ty.ret_types,
          expected_fn.ret_types,
        )
      };
    }

    true
  }
}
