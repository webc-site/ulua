use ulua_ast::records::location::Location;

use crate::type_aliases::{l_value::LValue, type_id::TypeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsAPredicate {
  pub lvalue: LValue,
  pub location: Location,
  pub ty: TypeId,
}
