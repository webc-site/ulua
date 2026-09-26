use ulua_ast::records::location::Location;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct AnnotationTypesAtLocation {
  pub location: Location,
  pub resolved_ty: TypeId,
}
