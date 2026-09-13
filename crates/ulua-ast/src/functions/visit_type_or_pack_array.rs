use crate::{
  records::{ast_array::AstArray, ast_type_or_pack::AstTypeOrPack, ast_visitor::AstVisitor},
  visit::{ast_type_pack_visit, ast_type_visit},
};

pub(crate) fn visit_type_or_pack_array(
  visitor: &mut dyn AstVisitor,
  array_of_type_or_pack: AstArray<AstTypeOrPack>,
) {
  for param in array_of_type_or_pack.as_slice() {
    if !param.r#type.is_null() {
      unsafe {
        ast_type_visit(param.r#type, visitor);
      }
    } else if !param.type_pack.is_null() {
      unsafe {
        ast_type_pack_visit(param.type_pack, visitor);
      }
    }
  }
}
