//! Faithful port of `TypeChecker2::checkForTypeFunctionInhabitance`
//! (TypeChecker2.cpp:496-508).
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  functions::reduce_type_functions_type_function::reduce_type_functions,
  records::{type_checker_2::TypeChecker2, type_function_context::TypeFunctionContext},
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn check_for_type_function_inhabitance(
    &mut self,
    instance: TypeId,
    location: Location,
  ) -> TypeId {
    if self.seen_type_function_instances.find(&instance).is_some() {
      return instance;
    }
    self.seen_type_function_instances.insert(instance);

    // TypeFunctionContext context{NotNull{&module->internalTypes}, builtinTypes,
    //     stack.back(), NotNull{&normalizer}, typeFunctionRuntime, ice, limits, subtyping};
    let context = {
      let arena = unsafe { NonNull::new_unchecked(&mut (*self.module).internal_types) };
      let builtins = unsafe { NonNull::new_unchecked(self.builtin_types) };
      let scope = unsafe { NonNull::new_unchecked(*self.stack.last().unwrap()) };
      let normalizer = unsafe { NonNull::new_unchecked(&mut self.normalizer) };
      let type_function_runtime = unsafe { NonNull::new_unchecked(self.type_function_runtime) };
      let ice = unsafe { NonNull::new_unchecked(self.ice) };
      let limits = unsafe { NonNull::new_unchecked(self.limits) };
      let subtyping = unsafe { NonNull::new_unchecked(self.subtyping) };
      TypeFunctionContext::from_components(
        arena,
        builtins,
        scope,
        normalizer,
        type_function_runtime,
        ice,
        limits,
        subtyping,
      )
    };

    let mut context = context;
    let errors =
      reduce_type_functions(instance, location, NonNull::from(&mut context), true).errors;

    if !self.is_error_suppressing_location_type_id(location, instance) {
      self.report_errors(errors);
    }
    instance
  }
}
