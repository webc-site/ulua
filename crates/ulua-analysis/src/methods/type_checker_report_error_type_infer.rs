//! @interface-stub
use alloc::sync::Arc;

use ulua_ast::enums::mode::Mode;

use crate::records::{module::Module, type_checker::TypeChecker, type_error::TypeError};
impl TypeChecker {
  pub fn report_error_type_error(&mut self, error: &TypeError) {
    let module = self.current_module.as_ref().unwrap();

    if module.mode == Mode::NoCheck {
      return;
    }

    unsafe {
      let module = Arc::as_ptr(module) as *mut Module;
      (*module).errors.push(error.clone());
      (*module).errors.last_mut().unwrap().module_name = (*module).name.clone();
    }
  }
}
