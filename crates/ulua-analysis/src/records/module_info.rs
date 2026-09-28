use crate::type_aliases::module_name_type::ModuleName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ModuleInfo {
  pub name: ModuleName,
  pub optional: bool,
}
