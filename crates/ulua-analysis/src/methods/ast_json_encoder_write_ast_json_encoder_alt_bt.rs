use crate::records::{ast_type::AstType, ast_type_pack::AstTypePack};

impl AstJsonEncoder {
  pub fn write_ast_type_or_pack(&mut self, node: &AstTypeOrPack) {
    if !node.r#type.is_null() {
      self.visit_ast_type_group(node.r#type as *mut AstType);
    } else {
      self.visit_ast_type_pack(node.type_pack as *mut AstTypePack);
    }
  }
}
