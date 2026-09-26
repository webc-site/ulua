use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{
    anyification::Anyification, arena_handle::Handle,
    normalization_too_complex::NormalizationTooComplex, type_checker::TypeChecker,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn anyify_type_id_location(&mut self, ty: TypeId, location: Location) -> TypeId {
    let arena = Handle::from_mut(unsafe {
      &mut (*arc_as_mut(self.expect_current_module())).internal_types
    });
    let mut anyification =
      Anyification::new(arena, self.builtin_types, self.any_type, self.any_type_pack);
    let any = anyification.base.substitute_type_id(ty);
    if anyification.normalization_too_complex {
      self.report_error_location_type_error_data(
        &location,
        NormalizationTooComplex::default().into(),
      );
    }
    if let Some(result) = any {
      result
    } else {
      self
        .report_error_location_type_error_data(&location, UnificationTooComplex::default().into());
      self.error_recovery_type_type_id(self.any_type)
    }
  }

  pub fn anyify_type_pack_id_location(&mut self, ty: TypePackId, location: Location) -> TypePackId {
    let arena = Handle::from_mut(unsafe {
      &mut (*arc_as_mut(self.expect_current_module())).internal_types
    });
    let mut anyification =
      Anyification::new(arena, self.builtin_types, self.any_type, self.any_type_pack);
    let any = anyification.base.substitute_type_pack_id(ty);
    if let Some(any) = any {
      any
    } else {
      self
        .report_error_location_type_error_data(&location, UnificationTooComplex::default().into());
      self.error_recovery_type_pack_type_pack_id(self.any_type_pack)
    }
  }
}
