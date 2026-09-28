use alloc::vec::Vec;

use crate::type_aliases::module_name_type::ModuleName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleHasCyclicDependency {
  pub(crate) cycle: Vec<ModuleName>,
}

impl ModuleHasCyclicDependency {
  pub const fn new(cycle: Vec<ModuleName>) -> Self {
    Self { cycle }
  }

  pub fn cycle(&self) -> &[ModuleName] {
    &self.cycle
  }
}
