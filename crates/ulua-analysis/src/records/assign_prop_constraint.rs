use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignPropConstraint {
  pub(crate) lhs_type: TypeId,
  pub(crate) prop_name: String,
  pub(crate) rhs_type: TypeId,
  pub(crate) prop_location: Option<Location>,
  pub(crate) prop_type: TypeId,
  pub(crate) decrement_prop_count: bool,
}

impl AssignPropConstraint {
  pub fn lhs_type(&self) -> TypeId {
    self.lhs_type
  }

  pub fn prop_name(&self) -> &str {
    &self.prop_name
  }

  pub fn rhs_type(&self) -> TypeId {
    self.rhs_type
  }

  pub fn prop_location(&self) -> &Option<Location> {
    &self.prop_location
  }

  pub fn prop_type(&self) -> TypeId {
    self.prop_type
  }

  pub fn decrement_prop_count(&self) -> bool {
    self.decrement_prop_count
  }
}
