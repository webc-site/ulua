use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::type_aliases::module_name_type::ModuleName;
#[derive(Debug, Clone)]
pub struct RequireCycle {
  pub location: Location,
  pub path: Vec<ModuleName>,
}
