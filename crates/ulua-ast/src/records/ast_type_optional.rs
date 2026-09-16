use crate::{
  records::ast_type::AstType,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeOptional {
  pub base: AstType,
  pub type_: *mut AstType,
}

impl AstNodeClass for AstTypeOptional {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeOptional");
}
