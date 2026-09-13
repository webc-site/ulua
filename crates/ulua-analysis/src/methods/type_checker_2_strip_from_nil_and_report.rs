//! Faithful port of `TypeChecker2::stripFromNilAndReport` (TypeChecker2.cpp:1883-1910).
use ulua_ast::records::location::Location;

use crate::{
  enums::value::Value,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil,
    should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    normalization_too_complex::NormalizationTooComplex, optional_value_access::OptionalValueAccess,
    type_checker_2::TypeChecker2, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
    let ty = follow_type_id(ty);

    // if (auto utv = get<UnionType>(ty))
    //     if (!std::any_of(begin(utv), end(utv), isNil)) return ty;
    if let Some(utv) = get_type_id::<UnionType>(ty)
      && !utv.options.iter().any(|&opt| is_nil(opt))
    {
      return ty;
    }

    if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
      match Value::from(unsafe { should_suppress_errors(&mut self.normalizer, ty) }) {
        Value::Suppress => {}
        Value::NormalizationFailed => {
          self.report_error_type_error_data_location(
            NormalizationTooComplex::default().into(),
            location,
          );
          // [[fallthrough]]
          self.report_error_type_error_data_location(
            OptionalValueAccess { optional: ty }.into(),
            location,
          );
        }
        Value::DoNotSuppress => {
          self.report_error_type_error_data_location(
            OptionalValueAccess { optional: ty }.into(),
            location,
          );
        }
      }

      return follow_type_id(stripped_union);
    }

    ty
  }
}
