use ulua_ast::records::location::Location;

use crate::type_aliases::{l_value::LValue, type_id::TypeId};

#[derive(Debug, Clone)]
pub struct EqPredicate {
  pub lvalue: LValue,
  pub ty: TypeId,
  pub location: Location,
}
