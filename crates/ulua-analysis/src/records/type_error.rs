use ulua_ast::records::location::Location;

use crate::type_aliases::{module_name_type::ModuleName, type_error_data::TypeErrorData};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
  pub location: Location,
  pub module_name: ModuleName,
  pub data: TypeErrorData,
}

unsafe impl Send for TypeError {}
unsafe impl Sync for TypeError {}
