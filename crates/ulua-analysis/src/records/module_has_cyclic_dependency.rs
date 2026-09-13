use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleHasCyclicDependency {
  pub(crate) cycle: Vec<String>,
}

impl ModuleHasCyclicDependency {
  pub const fn new(cycle: Vec<String>) -> Self {
    Self { cycle }
  }
}

impl ModuleHasCyclicDependency {
  pub fn cycle(&self) -> &[String] {
    &self.cycle
  }
}
