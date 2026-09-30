use ulua_ast::records::location::Location;

use crate::type_aliases::module_name_type::ModuleName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeErrorSummary {
  pub location: Location,
  pub module_name: ModuleName,
  pub code: i32,
}
