use ulua_ast::records::ast_type_intersection::AstTypeIntersection;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_type_intersection(
    &mut self,
    intersection_type: *mut AstTypeIntersection,
  ) {
    unsafe {
      let types = (*intersection_type).types;
      for &ty in types.as_slice() {
        self.visit_ast_type(ty);
      }
    }
  }
}
