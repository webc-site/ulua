use core::mem::take;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    normalization_too_complex::NormalizationTooComplex, scope::Scope,
    subtyping_result::SubtypingResult, type_checker_2::TypeChecker2,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  pub fn test_is_subtype_type_id_type_id_location(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
  ) -> bool {
    let scope: *mut Scope = self.find_innermost_scope(location);
    let mut r: SubtypingResult = unsafe { &mut *self.subtyping }
      .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope);

    if r.is_error_suppressing {
      return r.is_subtype;
    }

    for error in &mut r.errors {
      error.location = location;
    }

    self.report_errors(take(&mut r.errors));

    if r.normalization_too_complex {
      self.report_error_type_error_data_location(
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        &location,
      );
    }

    if !r.is_subtype {
      self.explain_error_type_id_type_id_location_subtyping_result(sub_ty, super_ty, location, &r);
    }

    r.is_subtype
  }
}
