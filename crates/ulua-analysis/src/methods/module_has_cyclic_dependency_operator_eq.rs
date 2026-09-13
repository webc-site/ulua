use crate::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;

impl ModuleHasCyclicDependency {
  #[inline]
  pub fn operator_eq(&self, rhs: &ModuleHasCyclicDependency) -> bool {
    self.cycle == rhs.cycle
  }
}
