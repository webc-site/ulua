use ulua_ast::{enums::mode::Mode, records::location::Location};
use ulua_common::macros::{
  luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
};

use crate::{
  enums::type_file_resolver::Type,
  functions::{first::first, get_type_pack_id::get_type_pack_id},
  records::{
    illegal_require::IllegalRequire, module_info::ModuleInfo, type_checker::TypeChecker,
    type_error::TypeError, unknown_require::UnknownRequire,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_require(
    &mut self,
    scope: &ScopePtr,
    module_info: &ModuleInfo,
    location: &Location,
  ) -> TypeId {
    LUAU_TIMETRACE_SCOPE!("TypeChecker::checkRequire", "TypeChecker");
    LUAU_TIMETRACE_ARGUMENT!("moduleInfo", module_info.name.as_str());

    if module_info.name.is_empty() {
      if let Some(ref current_module) = self.current_module
        && current_module.mode == Mode::Strict
      {
        let error = TypeError::type_error_location_type_error_data(
          *location,
          UnknownRequire::new(module_info.name.clone()).into(),
        );
        self.report_error_type_error(&error);
        return self.error_recovery_type_type_id(self.any_type);
      }
      return self.any_type;
    }

    // Types of requires that transitively refer to current module have to be replaced with 'any'
    for require_cycle in &self.require_cycles {
      if !require_cycle.path.is_empty() && require_cycle.path[0] == module_info.name {
        return self.any_type;
      }
    }

    let module = unsafe { ((*self.resolver).vtable.get_module)(self.resolver, &module_info.name) };
    if module.is_none() {
      // There are two reasons why we might fail to find the module:
      // either the file does not exist or there's a cycle. If there's a cycle
      // we will already have reported the error.
      if unsafe { !((*self.resolver).vtable.module_exists)(self.resolver, &module_info.name) }
        && !module_info.optional
      {
        let human_readable_name = unsafe {
          ((*self.resolver).vtable.get_human_readable_module_name)(self.resolver, &module_info.name)
        };
        let error = TypeError::type_error_location_type_error_data(
          *location,
          UnknownRequire::new(human_readable_name).into(),
        );
        self.report_error_type_error(&error);
      }
      return self.error_recovery_type_scope_ptr(scope);
    }

    let module = module.unwrap();
    if module.r#type != Type::Module {
      let error = TypeError::type_error_location_type_error_data(
        *location,
        IllegalRequire::new(
          module.human_readable_name.clone(),
          "Module is not a ModuleScript.  It cannot be required.".to_string(),
        )
        .into(),
      );
      self.report_error_type_error(&error);
      return self.error_recovery_type_scope_ptr(scope);
    }

    if module
      .errors
      .iter()
      .any(|error| matches!(error.data, TypeErrorData::SyntaxError(_)))
    {
      return self.error_recovery_type_scope_ptr(scope);
    }

    let module_pack = module.return_type;

    if !get_type_pack_id::<ErrorTypePack>(module_pack).is_none() {
      return self.error_recovery_type_scope_ptr(scope);
    }

    let module_type = first(module_pack, true);
    if module_type.is_none() {
      let error = TypeError::type_error_location_type_error_data(
        *location,
        IllegalRequire::new(
          module.human_readable_name.clone(),
          "Module does not return exactly 1 value.  It cannot be required.".to_string(),
        )
        .into(),
      );
      self.report_error_type_error(&error);
      return self.error_recovery_type_scope_ptr(scope);
    }

    module_type.unwrap()
  }
}
