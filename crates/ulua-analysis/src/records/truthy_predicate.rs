use ulua_ast::records::location::Location;

use crate::type_aliases::l_value::LValue;

#[derive(Debug, Clone)]
pub struct TruthyPredicate {
  pub lvalue: LValue,
  pub location: Location,
}
