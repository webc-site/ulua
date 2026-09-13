use core::ffi::CStr;

use ulua_ast::records::{ast_array::AstArray, ast_type_reference::AstTypeReference};
#[derive(Debug, Clone)]
pub struct BuiltinAstTypes {
  pub boolean_type: AstTypeReference,
  pub number_type: AstTypeReference,
  pub integer_type: AstTypeReference,
  pub string_type: AstTypeReference,
  pub vector_type: AstTypeReference,
  pub host_vector_type: AstTypeReference,
}

impl Default for BuiltinAstTypes {
  fn default() -> Self {
    use ulua_ast::records::{
      ast_name::AstName, ast_type_reference::AstTypeReference, location::Location,
      position::Position,
    };

    let loc = Location::new(Position::new(0, 0), Position::new(0, 0));
    let empty_arr = AstArray::default();

    let make_ref = |name: &'static CStr| {
      AstTypeReference::new(
        loc,
        None,
        AstName::from_c_str(name),
        None,
        loc,
        false,
        empty_arr,
      )
    };

    Self {
      boolean_type: make_ref(c"boolean"),
      number_type: make_ref(c"number"),
      integer_type: make_ref(c"integer"),
      string_type: make_ref(c"string"),
      vector_type: make_ref(c"vector"),
      host_vector_type: make_ref(c"vector"),
    }
  }
}
