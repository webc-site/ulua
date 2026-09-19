use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::follow_type::follow_type_id, records::non_strict_type_checker::NonStrictTypeChecker,
  type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lookup_type(&mut self, expr: *mut AstExpr) -> TypeId {
    let module = unsafe { &*self.module };

    if let Some(ty) = module.ast_types.find(&(expr as *const AstExpr)) {
      let location = unsafe { (*expr).base.location };
      self.check_for_type_function_inhabitance(follow_type_id(*ty), location)
    } else if let Some(tp) = module.ast_type_packs.find(&(expr as *const AstExpr)) {
      let location = unsafe { (*expr).base.location };
      let flattened = self.flatten_pack(*tp);
      self.check_for_type_function_inhabitance(flattened, location)
    } else {
      unsafe { (*self.builtin_types).any_type }
    }
  }
}
