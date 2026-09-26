use ulua_ast::records::location::Location;

use crate::type_aliases::name_type::Name;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionArgument {
  pub name: Name,
  pub location: Location,
}
