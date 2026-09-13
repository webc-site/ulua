use ulua_ast::records::ast_type_list::AstTypeList;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  pub fn visit_ast_type_list(&mut self, list: &mut AstTypeList) {
    for &t in list.types.as_slice() {
      self.visit_ast_type(t);
    }

    if !list.tail_type.is_null() {
      self.visit_ast_type_pack(list.tail_type);
    }
  }
}
