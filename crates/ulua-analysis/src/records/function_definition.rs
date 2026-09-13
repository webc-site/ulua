use ulua_ast::records::location::Location;

use crate::type_aliases::module_name_type::ModuleName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDefinition {
  pub(crate) definition_module_name: Option<ModuleName>,
  pub(crate) definition_location: Location,
  pub(crate) vararg_location: Option<Location>,
  pub(crate) original_name_location: Location,
}

impl FunctionDefinition {
  pub fn definition_module_name(&self) -> Option<&ModuleName> {
    self.definition_module_name.as_ref()
  }

  pub fn definition_location(&self) -> Location {
    self.definition_location
  }

  pub fn vararg_location(&self) -> Option<Location> {
    self.vararg_location
  }

  pub fn original_name_location(&self) -> Location {
    self.original_name_location
  }
}
