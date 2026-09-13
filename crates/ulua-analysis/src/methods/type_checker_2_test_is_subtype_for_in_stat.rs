use core::mem::take;

use ulua_ast::records::ast_stat_for_in::AstStatForIn;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    function_type::FunctionType, normalization_too_complex::NormalizationTooComplex,
    type_checker_2::TypeChecker2,
  },
  type_aliases::type_id::TypeId,
};
impl TypeChecker2 {
  pub fn test_is_subtype_for_in_stat(
    &mut self,
    iter_func: TypeId,
    prospective_func: TypeId,
    for_in_stat: &AstStatForIn,
  ) {
    LUAU_ASSERT!(get_type_id::<FunctionType>(follow_type_id(iter_func)).is_some());
    LUAU_ASSERT!(get_type_id::<FunctionType>(follow_type_id(prospective_func)).is_some());

    // SAFETY: for_in_stat.values 至少含一个迭代函数表达式（语法保证）。
    let iter_func_location = unsafe { &**for_in_stat.values.data.add(0) }.base.location;

    let scope = self.find_innermost_scope(iter_func_location);
    // SAFETY: self.subtyping 由构造方保证有效（C++ 同契约）。
    let mut r = unsafe {
      (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
        iter_func,
        prospective_func,
        scope,
      )
    };

    if !self.is_error_suppressing_location_type_id(iter_func_location, iter_func) {
      for e in &mut r.errors {
        e.location = iter_func_location;
      }
    }

    self.report_errors(take(&mut r.errors));

    if r.normalization_too_complex {
      self.report_error_type_error_data_location(
        NormalizationTooComplex::default().into(),
        &iter_func_location,
      );
    }

    if r.is_subtype {
      return;
    }

    self.explain_error_type_id_type_id_location_subtyping_result(
      iter_func,
      prospective_func,
      iter_func_location,
      &r,
    );
  }
}
