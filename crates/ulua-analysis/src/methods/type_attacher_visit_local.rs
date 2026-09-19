use ulua_ast::records::ast_local::AstLocal;

use crate::{
  records::{symbol::Symbol, type_attacher::TypeAttacher},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeAttacher {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_local(&mut self, local: *mut AstLocal) -> bool {
    // C++ `AstType* annotation = local->annotation;`
    let annotation = unsafe { (*local).annotation };
    if annotation.is_null() {
      // C++ `if (auto scope = getScope(local->location))` — the Rust
      // `get_scope` always resolves an enclosing scope (the global scope
      // encloses any location), so it is unconditionally usable here.
      let location = unsafe { (*local).location };
      let scope: ScopePtr = self.get_scope(&location);
      // C++ `if (auto result = scope->lookup(local))` — the implicit
      // `Symbol(AstLocal*)` conversion then `lookup(Symbol)`.
      let result = scope.lookup_symbol(Symbol::from_local(local));
      if let Some(type_id) = result {
        let annotated = self.type_ast(Some(type_id));
        unsafe {
          (*local).annotation = annotated;
        }
      }
    }
    true
  }
}
