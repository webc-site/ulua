use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr_call::AstExprCall, location::Location};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, size_type_pack::size,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    extra_information::ExtraInformation, function_type::FunctionType, generic_error::GenericError,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn report_overload_resolution_error(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprCall,
    ret_pack: TypePackId,
    arg_pack: TypePackId,
    arg_locations: &[Location],
    overloads: &[TypeId],
    overloads_that_match_arg_count: &[TypeId],
    errors: &mut [OverloadErrorEntry],
  ) {
    if overloads.len() == 1 {
      let error = errors
        .first_mut()
        .expect("single failed overload has errors");
      error.log.commit();
      let reported_errors = error.errors.clone();

      self.report_errors(&reported_errors);
      return;
    }

    let mut overload_types = overloads_that_match_arg_count.to_vec();
    if overloads_that_match_arg_count.is_empty() {
      self.report_error_location_type_error_data(
        &expr.base.base.location,
        TypeErrorData::GenericError(GenericError::new(format!(
          "No overload for function accepts {} arguments.",
          unsafe { size(arg_pack, null_mut()) }
        ))),
      );

      overload_types = overloads.to_vec();
    } else {
      let overload = overloads_that_match_arg_count[0];
      overload_types.retain(|ty| *ty != overload);

      let ftv = get_type_id::<FunctionType>(overload);
      LUAU_ASSERT!(ftv.is_some());
      // 指针相等比较（C++ errors[i].fnTy == ftv）。
      let ftv_ptr = ftv.unwrap() as *const FunctionType;

      let error_index = errors.iter().position(|e| e.fn_ty == ftv_ptr);
      LUAU_ASSERT!(error_index.is_some());

      let error = &mut errors[error_index.unwrap()];
      error.log.commit();
      let reported_errors = error.errors.clone();

      self.report_errors(&reported_errors);

      if overloads_that_match_arg_count.len() == 1 {
        return;
      }
    }

    let mut s = String::new();
    for (i, overload) in overload_types.iter().enumerate() {
      let overload = follow_type_id(*overload);
      let mut state = self.mk_unifier(scope, &expr.base.base.location);

      if let Some(ftv) = get_type_id::<FunctionType>(overload) {
        self.check_argument_list(
          scope,
          unsafe { &*expr.func },
          &mut state,
          ret_pack,
          ftv.ret_types,
          &Vec::new(),
        );
        self.check_argument_list(
          scope,
          unsafe { &*expr.func },
          &mut state,
          arg_pack,
          ftv.arg_types,
          arg_locations,
        );
      }

      if state.errors.is_empty() {
        state.log.commit();
      }

      if i > 0 {
        s.push_str("; ");
      }

      if i > 0 && i == overload_types.len() - 1 {
        s.push_str("and ");
      }

      s.push_str(&to_string_type_id(overload));
    }

    let message = if overloads_that_match_arg_count.is_empty() {
      String::from("Available overloads: ") + &s
    } else {
      String::from("Other overloads are also not viable: ") + &s
    };

    self.report_error_location_type_error_data(
      unsafe { &(*expr.func).base.location },
      TypeErrorData::ExtraInformation(ExtraInformation::new(message)),
    );
  }
}
