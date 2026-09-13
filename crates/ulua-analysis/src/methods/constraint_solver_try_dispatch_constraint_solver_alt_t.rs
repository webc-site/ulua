use ulua_common::FFlag;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    contains_any_generic_deprecated::ContainsAnyGenericDeprecated, free_type::FreeType,
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
    let (Some(expected_fn), Some(fn_ty)) = (
      get_type_id::<FunctionType>(follow_type_id(c.expected_function_type)),
      get_type_id::<FunctionType>(follow_type_id(c.function_type)),
    ) else {
      return true;
    };

    if FFlag::LuauInstantiateFunctionTypeBeforePush.get() {
      // instantiate is not yet translated; stubbing the logic as per the source comment
      // "NOTE: This logic could probably be combined with that of FunctionCheckConstraint"
      // Since instantiate is not available, we skip this block and rely on the fallback behavior
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
        if !FFlag::LuauConstraintGraph.get() {
          self.deprecate_d_shift_references(params_current, *expected_params.operator_deref());
        }
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(constraint, params_current, {
            *expected_params.operator_deref()
          })
        };
      }
      expected_params.operator_inc();
      params.operator_inc();
    }

    let mut idx: usize = 0;
    while idx < unsafe { (*c.expr).args.size }
      && !expected_params.operator_eq(&expected_params_end)
      && !params.operator_eq(&params_end)
    {
      let arg = unsafe { *(*c.expr).args.data.add(idx) };
      let annotation = unsafe { (*arg).annotation };
      let params_current = *params.operator_deref();
      let free_type = get_type_id::<FreeType>(follow_type_id(params_current));

      if annotation.is_null()
        && free_type.is_some()
        && (FFlag::LuauInstantiateFunctionTypeBeforePush.get()
          || !ContainsAnyGenericDeprecated::has_any_generic(*expected_params.operator_deref()))
      {
        if !FFlag::LuauConstraintGraph.get() {
          self.deprecate_d_shift_references(params_current, *expected_params.operator_deref());
        }
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(constraint, params_current, {
            *expected_params.operator_deref()
          })
        };
      }

      expected_params.operator_inc();
      params.operator_inc();
      idx += 1;
    }

    if unsafe { (*c.expr).return_annotation.is_null() }
      && get_type_pack_id::<FreeTypePack>(fn_ty.ret_types).is_some()
      && (FFlag::LuauInstantiateFunctionTypeBeforePush.get()
        || !ContainsAnyGenericDeprecated::has_any_generic_type_pack_id(expected_fn.ret_types))
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
