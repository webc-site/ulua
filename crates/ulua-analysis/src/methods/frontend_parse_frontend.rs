use alloc::vec::Vec;

use crate::{
  records::{frontend::Frontend, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn parse_module_name(&mut self, name: &ModuleName) {
    ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE!("Frontend::parse", "Frontend");
    ulua_common::macros::luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    if self.get_check_result(name, false, false).is_some() {
      return;
    }

    let mut build_queue: Vec<ModuleName> = Vec::new();
    self.parse_graph(&mut build_queue, name, &TypeCheckLimits::default(), false);
  }
}
