use crate::{
  records::null_module_resolver::NullModuleResolver,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

impl NullModuleResolver {
  /// C++ `const ModulePtr getModule(const ModuleName&) const override { return nullptr; }`
  /// (ModuleResolver.h:57). The nullable `ModulePtr` is modeled as
  /// `Option<ModulePtr>`, so the null answer is `None`.
  pub fn get_module(&self, _module_name: &ModuleName) -> Option<ModulePtr> {
    None
  }
}
