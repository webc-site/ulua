use alloc::string::String;

use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    // C++ `if (!frontend) return name;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    match self.frontend_ref() {
      Some(frontend) => frontend
        .file_resolver_ref()
        .get_human_readable_module_name(module_name),
      None => module_name.to_string(),
    }
  }
}
