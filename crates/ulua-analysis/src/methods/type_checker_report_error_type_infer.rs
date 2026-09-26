use ulua_ast::{enums::mode::Mode, records::location::Location};

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{type_checker::TypeChecker, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorData,
};

impl TypeChecker {
  pub fn report_error_type_error(&mut self, error: &TypeError) {
    let module = self.expect_current_module();

    if module.mode == Mode::NoCheck {
      return;
    }

    unsafe {
      let module = arc_as_mut(module);
      (*module).errors.push(error.clone());
      // Safety: 紧邻上方 push 之后取尾，必命中（cpp errors.back() 同位）。
      (*module)
        .errors
        .last_mut()
        .expect("紧邻 push 之后取尾，必命中")
        .module_name = (*module).name.clone();
    }
  }

  pub fn report_error_location_type_error_data(
    &mut self,
    _location: &Location,
    _error_data: TypeErrorData,
  ) {
    let error = TypeError::type_error_location_type_error_data(*_location, _error_data);
    self.report_error_type_error(&error);
  }
}
