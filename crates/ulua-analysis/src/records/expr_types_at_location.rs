use ulua_ast::records::location::Location;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprTypesAtLocation {
  pub location: Location,
  pub ty: TypeId,
  pub expected_ty: Option<TypeId>,
}
