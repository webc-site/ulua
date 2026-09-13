use ulua_ast::records::{
  ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
  ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
};

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  pub fn visit_ast_type_pack(&mut self, pack: *mut AstTypePack) {
    if pack.is_null() {
      return;
    }

    unsafe {
      let node = pack as *mut AstNode;
      if (*node).is::<AstTypePackExplicit>() {
        self.visit_ast_type_pack_explicit(pack as *mut AstTypePackExplicit);
      } else if (*node).is::<AstTypePackVariadic>() {
        self.visit_ast_type_pack_variadic(pack as *mut AstTypePackVariadic);
      } else if (*node).is::<AstTypePackGeneric>() {
        // ok
      } else {
        // unreachable
      }
    }
  }
}
