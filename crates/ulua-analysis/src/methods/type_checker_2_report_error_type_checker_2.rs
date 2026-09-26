use ulua_ast::records::location::Location;

use crate::{
  functions::diagnose_missing_table_key::diagnose_missing_table_key,
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
      diagnose_missing_table_key(&utk, &mut data);
    }

    // module->errors.emplace_back(location, module->name, std::move(data));
    // 经 `module_mut` 访问器收口（module 字段裸解引用契约集中在访问器）。
    let module_name = self.module_ref().name.clone();
    self
      .module_mut()
      .errors
      .push(TypeError::type_error_location_module_name_type_error_data(
        *location,
        module_name,
        data,
      ));

    // if (logger) logger->captureTypeCheckError(module->errors.back());
    // `Option<Handle>` 承载原 C++ `DcrLogger*` 判空语义，一一对应；
    // `capture_type_check_error` 只读取刚 push 的栈顶错误。
    if let Some(logger) = self.logger {
      let last = self
        .module_ref()
        .errors
        .last()
        .expect("errors 刚 push 过，last() 必非空");
      logger.get_mut().capture_type_check_error(last);
    }
  }

  pub fn report_error_type_error(&mut self, e: TypeError) {
    self.report_error_type_error_data_location(e.data, &e.location);
  }
}
