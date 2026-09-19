use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, module_has_cyclic_dependency::ModuleHasCyclicDependency,
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

      if let Some(file_resolver) = self.file_resolver {
        // SAFETY: 指针由调用方保证指向存活 resolver。
        let readable = unsafe { (*file_resolver).get_human_readable_module_name(name) };
        s.push_str(&readable);
      } else {
        s.push_str(name);
      }
    }

    s
  }
}
