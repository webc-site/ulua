use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn module_exists(&self, module_name: &ModuleName) -> bool {
    // C++ `if (!frontend) return false;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    self
      .frontend_ref()
      .is_some_and(|frontend| frontend.source_nodes.contains_key(module_name))
  }
}
