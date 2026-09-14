//! Faithful port of `TypeChecker2::reportError(TypeErrorData, const Location&)`
//! (TypeChecker2.cpp:3449-3458).
use ulua_ast::records::location::Location;

use crate::{
  records::{
    type_checker_2::TypeChecker2, type_error::TypeError, unknown_property::UnknownProperty,
  },
  type_aliases::type_error_data::{TypeErrorData, TypeErrorDataMember},
};

impl TypeChecker2 {
  pub fn report_error_type_error_data_location(
    &mut self,
    mut data: TypeErrorData,
    location: &Location,
  ) {
    // if (auto utk = get_if<UnknownProperty>(&data)) diagnoseMissingTableKey(utk, data);
    if let Some(utk) = UnknownProperty::get_if(&data) {
      // C++ holds a pointer into `data` while also mutating `data`;
      // clone the property so the borrow checker is satisfied without
      // changing behaviour (diagnoseMissingTableKey only reads `utk`).
      let utk = utk.clone();
      self.diagnose_missing_table_key(&utk, &mut data);
    }

    // module->errors.emplace_back(location, module->name, std::move(data));
    let module_name = unsafe { (*self.module).name.clone() };
    unsafe {
      (*self.module)
        .errors
        .push(TypeError::type_error_location_module_name_type_error_data(
          *location,
          module_name,
          data,
        ));
    }

    // if (logger) logger->captureTypeCheckError(module->errors.back());
    if !self.logger.is_null() {
      let last = unsafe { (*self.module).errors.last().unwrap() };
      unsafe { (*self.logger).capture_type_check_error(last) };
    }
  }
}
