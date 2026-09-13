use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, file_resolver::FileResolver,
  module_has_cyclic_dependency::ModuleHasCyclicDependency,
};

impl ErrorConverter {
  pub fn operator_call_33(&self, e: &ModuleHasCyclicDependency) -> String {
    if e.cycle().is_empty() {
      return String::from("Cyclic module dependency detected");
    }

    let mut s = String::from("Cyclic module dependency: ");

    let mut first = true;
    for name in e.cycle() {
      if first {
        first = false;
      } else {
        s.push_str(" -> ");
      }

      if !self.file_resolver.is_null() {
        unsafe {
          s.push_str(&FileResolver::get_human_readable_module_name(
            self.file_resolver,
            name,
          ));
        }
      } else {
        s.push_str(name);
      }
    }

    s
  }
}
