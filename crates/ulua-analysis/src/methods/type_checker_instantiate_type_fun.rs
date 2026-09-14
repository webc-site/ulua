use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    apply_type_function::ApplyTypeFunction, function_type::FunctionType, module::Module,
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

    let module_ptr =
      Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
    let arena = unsafe { &mut (*module_ptr).internal_types as *mut _ };
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

    if let Some(ftv) = get_mutable_type_id::<FunctionType>(instantiated) {
      ftv.generics.clear();
      ftv.generic_packs.clear();
    }

    instantiated
  }
}
