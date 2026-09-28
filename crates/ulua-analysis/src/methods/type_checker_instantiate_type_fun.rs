use ulua_ast::records::location::Location;

use crate::{
  functions::{arc_as_mut::arc_as_mut, get_mutable_type},
  records::{
    apply_type_function::ApplyTypeFunction, arena_handle::Handle, function_type::FunctionType,
    recursive_restraint_violation::RecursiveRestraintViolation, type_checker::TypeChecker,
    type_fun::TypeFun, unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub fn instantiate_type_fun(
    &mut self,
    scope: &ScopePtr,
    tf: &TypeFun,
    type_params: &[TypeId],
    type_pack_params: &[TypePackId],
    location: &Location,
  ) -> TypeId {
    if tf.type_params.is_empty() && tf.type_pack_params.is_empty() {
      return tf.r#type;
    }

    let module_ptr = arc_as_mut(self.expect_current_module());
    // Safety: internal_types 是模块独占的 TypeArena，取句柄后仅此借用。
    let arena = Handle::from_mut(unsafe { &mut (*module_ptr).internal_types });
    let mut apply_type_function = ApplyTypeFunction::new(arena);

    for (i, type_param) in tf.type_params.iter().enumerate() {
      if let Some(&argument) = type_params.get(i) {
        *apply_type_function
          .type_arguments
          .get_or_insert(type_param.ty) = argument;
      }
    }

    for (i, type_pack_param) in tf.type_pack_params.iter().enumerate() {
      if let Some(&argument) = type_pack_params.get(i) {
        *apply_type_function
          .type_pack_arguments
          .get_or_insert(type_pack_param.tp) = argument;
      }
    }

    let Some(instantiated) = apply_type_function.substitute_type_id(tf.r#type) else {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
      );
      return self.error_recovery_type_scope_ptr(scope);
    };

    if apply_type_function.encountered_forwarded_type {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::RecursiveRestraintViolation(RecursiveRestraintViolation::default()),
      );
      return self.error_recovery_type_scope_ptr(scope);
    }

    if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(instantiated) {
      ftv.generics.clear();
      ftv.generic_packs.clear();
    }

    instantiated
  }
}
