use alloc::string::{String, ToString};

use ulua_ast::records::location::Location;

use crate::{
  enums::type_file_resolver::Type,
  functions::{first::first, get_type_pack},
  records::{
    constraint_solver::ConstraintSolver, illegal_require::IllegalRequire, module_info::ModuleInfo,
    unknown_require::UnknownRequire,
  },
  type_aliases::{error_type_pack::ErrorTypePack, type_id::TypeId},
};

impl ConstraintSolver {
  pub fn resolve_module(&mut self, info: &ModuleInfo, location: &Location) -> TypeId {
    // Safety: `builtin_types` 在 ConstraintSolver 构造时从 `(*normalizer).builtin_types`
    // 接线——非空、指向 TypeChecker 会话的 BuiltinTypes，比 solver 长寿且类型检查期
    // 不再写入。提为一次共享借用统一取 error/any 等 Copy 句柄；其间 &mut self 调用
    // （report_*）只触及 errors 等其它字段，不经过该借用。
    let builtin_types = self.builtin_types.get();

    if info.name.is_empty() {
      self
        .report_error_type_error_data_location(UnknownRequire::new(String::new()).into(), location);
      return builtin_types.error_type;
    }

    for require_cycle in &self.require_cycles {
      if !require_cycle.path.is_empty() && require_cycle.path[0] == info.name {
        return builtin_types.any_type;
      }
    }

    let module = self.module_resolver_ref().get_module(&info.name);
    if module.is_none() {
      if !self.module_resolver_ref().module_exists(&info.name) && !info.optional {
        let human_readable_name = self
          .module_resolver_ref()
          .get_human_readable_module_name(&info.name);
        self.report_error_type_error_data_location(
          UnknownRequire::new(human_readable_name).into(),
          location,
        );
      }
      return builtin_types.error_type;
    }

    // Safety: 上方 is_none 分支已早返，module 至此必为 Some。
    let module = module.expect("上方 is_none 分支已早返，至此必为 Some");
    if module.r#type != Type::Module {
      self.report_error_type_error_data_location(
        IllegalRequire::new(
          module.human_readable_name.clone(),
          "Module is not a ModuleScript. It cannot be required.".to_string(),
        )
        .into(),
        location,
      );
      return builtin_types.error_type;
    }

    let module_pack = module.return_type;
    if get_type_pack::get::<ErrorTypePack>(module_pack).is_some() {
      return builtin_types.error_type;
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
      return builtin_types.error_type;
    }

    // Safety: 上方 is_none 分支已早返报错，至此必为 Some。
    module_type.expect("上方 is_none 分支已早返，至此必为 Some")
  }
}
