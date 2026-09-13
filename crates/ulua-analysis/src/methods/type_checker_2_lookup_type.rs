//! Faithful port of `TypeChecker2::lookupType` (TypeChecker2.cpp:522-536).
use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::follow_type::follow_type_id, records::type_checker_2::TypeChecker2,
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn lookup_type(&mut self, expr: &AstExpr) -> TypeId {
    // If a type isn't in the type graph, it probably means that a recursion limit was exceeded.
    // We'll just return any_type in these cases.  Typechecking against any is very fast and this
    // allows us not to think about this very much in the actual typechecking logic.
    let location = expr.base.location;

    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let ty = unsafe { &*self.module }
      .ast_types
      .find(&(expr as *const AstExpr))
      .copied();
    if let Some(ty) = ty {
      return self.check_for_type_function_inhabitance(follow_type_id(ty), location);
    }

    // SAFETY: 同上。
    let tp = unsafe { &*self.module }
      .ast_type_packs
      .find(&(expr as *const AstExpr))
      .copied();
    if let Some(tp) = tp {
      let flattened = self.flatten_pack(tp);
      return self.check_for_type_function_inhabitance(flattened, location);
    }

    // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
    unsafe { (*self.builtin_types).any_type }
  }
}
