use ulua_ast::records::location::Location;

use crate::type_aliases::{
  module_name_type::ModuleName, type_function_error_data::TypeFunctionErrorData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunctionError {
  pub(crate) location: Location,
  pub(crate) module_name: ModuleName,
  pub(crate) data: TypeFunctionErrorData,
}
