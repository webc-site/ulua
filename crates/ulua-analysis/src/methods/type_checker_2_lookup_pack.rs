use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::follow_type_pack::follow_type_pack_id, records::type_checker_2::TypeChecker2,
  type_aliases::type_pack_id::TypePackId,
};

impl TypeChecker2 {
  pub fn lookup_pack(&self, expr: *mut AstExpr) -> TypePackId {
    // If a type isn't in the type graph, it probably means that a recursion limit was exceeded.
    // We'll just return any_type in these cases.  Typechecking against any is very fast and this
    // allows us not to think about this very much in the actual typechecking logic.
    let module = unsafe { &*self.module };
    let tp = module.ast_type_packs.find(&(expr as *const AstExpr));
    if let Some(tp) = tp {
      unsafe { follow_type_pack_id(*tp) }
    } else {
      unsafe { (*self.builtin_types).any_type_pack }
    }
  }
}
