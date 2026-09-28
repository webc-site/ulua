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
