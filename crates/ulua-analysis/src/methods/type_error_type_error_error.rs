use alloc::string::String;
use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  enums::context_error::Context,
  records::{type_error::TypeError, type_mismatch::TypeMismatch},
  type_aliases::{module_name_type::ModuleName, type_error_data::TypeErrorData},
};

impl TypeError {
  pub fn new() -> Self {
    Self {
      location: Location::default(),
      module_name: ModuleName::new(),
      // cpp variant 默认首选项 TypeMismatch{nullptr, nullptr, ""}；
      // zeroed 会把 String/Option<Arc> 打成非法堆状态，改用安全构造（位等价）
      data: TypeErrorData::TypeMismatch(TypeMismatch {
        wanted_type: null(),
        given_type: null(),
        reason: String::new(),
        error: None,
        context: Context::COVARIANT,
      }),
    }
  }
}

impl Default for TypeError {
  fn default() -> Self {
    Self::new()
  }
}

impl TypeError {
  pub fn type_error_location_module_name_type_error_data(
    location: Location,
    module_name: ModuleName,
    data: TypeErrorData,
  ) -> Self {
    Self {
      location,
      module_name,
      data,
    }
  }

  pub fn type_error_location_type_error_data(location: Location, data: TypeErrorData) -> Self {
    Self {
      location,
      module_name: ModuleName::new(),
      data,
    }
  }
}
