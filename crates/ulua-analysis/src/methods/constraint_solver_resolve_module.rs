use alloc::string::{String, ToString};

use ulua_ast::records::location::Location;

use crate::{
  enums::type_file_resolver::Type,
  functions::{first::first, get_type_pack::get_type_pack_id},
  records::{
    constraint_solver::ConstraintSolver, illegal_require::IllegalRequire, module_info::ModuleInfo,
    unknown_require::UnknownRequire,
  },
  type_aliases::{error_type_pack::ErrorTypePack, type_error_data::TypeErrorData, type_id::TypeId},
};

impl ConstraintSolver {
  pub fn resolve_module(&mut self, info: &ModuleInfo, location: &Location) -> TypeId {
    if info.name.is_empty() {
      self
        .report_error_type_error_data_location(UnknownRequire::new(String::new()).into(), location);
      return unsafe { (*self.builtin_types).error_type };
    }

    for require_cycle in &self.require_cycles {
      if !require_cycle.path.is_empty() && require_cycle.path[0] == info.name {
        return unsafe { (*self.builtin_types).any_type };
      }
    }

    let module =
      unsafe { ((*self.module_resolver).vtable.get_module)(self.module_resolver, &info.name) };
    if module.is_none() {
      if unsafe {
        !((*self.module_resolver).vtable.module_exists)(self.module_resolver, &info.name)
      } && !info.optional
      {
        let human_readable_name = unsafe {
          ((*self.module_resolver)
            .vtable
            .get_human_readable_module_name)(self.module_resolver, &info.name)
        };
        self.report_error_type_error_data_location(
          UnknownRequire::new(human_readable_name).into(),
          location,
        );
      }
      return unsafe { (*self.builtin_types).error_type };
    }

    let module = module.unwrap();
    if module.r#type != Type::Module {
      self.report_error_type_error_data_location(
        IllegalRequire::new(
          module.human_readable_name.clone(),
          "Module is not a ModuleScript. It cannot be required.".to_string(),
        )
        .into(),
        location,
      );
      return unsafe { (*self.builtin_types).error_type };
    }

    if module
      .errors
      .iter()
      .any(|error| matches!(error.data, TypeErrorData::SyntaxError(_)))
    {
      return unsafe { (*self.builtin_types).error_type };
    }

    let module_pack = module.return_type;
    if !get_type_pack_id::<ErrorTypePack>(module_pack).is_none() {
      return unsafe { (*self.builtin_types).error_type };
    }

    let module_type = first(module_pack, true);
    if module_type.is_none() {
      self.report_error_type_error_data_location(
        IllegalRequire::new(
          module.human_readable_name.clone(),
          "Module does not return exactly 1 value. It cannot be required.".to_string(),
        )
        .into(),
        location,
      );
      return unsafe { (*self.builtin_types).error_type };
    }

    module_type.unwrap()
  }
}
