use crate::records::{
  ast_local::AstLocal, ast_name::AstName, ast_type::AstType, location::Location,
};

impl AstLocal {
  pub fn new(
    name: AstName,
    location: Location,
    shadow: *mut AstLocal,
    function_depth: usize,
    loop_depth: usize,
    annotation: *mut AstType,
    is_const: bool,
  ) -> Self {
    Self {
      name,
      location,
      shadow,
      function_depth,
      loop_depth,
      is_const,
      is_exported: false,
      annotation,
    }
  }
}

pub fn ast_local_ast_local(
  name: AstName,
  location: Location,
  shadow: *mut AstLocal,
  function_depth: usize,
  loop_depth: usize,
  annotation: *mut AstType,
  is_const: bool,
) -> AstLocal {
  AstLocal::new(
    name,
    location,
    shadow,
    function_depth,
    loop_depth,
    annotation,
    is_const,
  )
}
