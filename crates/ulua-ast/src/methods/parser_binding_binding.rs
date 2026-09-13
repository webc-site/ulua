use crate::records::{ast_type::AstType, binding::Binding, name::Name, position::Position};

impl Binding {
  pub fn new(
    name: Name,
    annotation: *mut AstType,
    colon_position: Position,
    is_const: bool,
  ) -> Self {
    Self {
      name,
      annotation,
      colon_position,
      is_const,
    }
  }
}

pub fn parser_binding_binding(
  name: Name,
  annotation: *mut AstType,
  colon_position: Position,
  is_const: bool,
) -> Binding {
  Binding::new(name, annotation, colon_position, is_const)
}
