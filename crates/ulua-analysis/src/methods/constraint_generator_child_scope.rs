use alloc::sync::Arc;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  records::{constraint_generator::ConstraintGenerator, module::Module, scope::Scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn child_scope(&mut self, node: *mut AstNode, parent: &ScopePtr) -> ScopePtr {
    let scope: ScopePtr = Arc::new(Scope::new(parent, 0));
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
    self
      .scopes
      .push((unsafe { (*node).location }, scope.clone()));

    unsafe {
      (*scope_raw).location = (*node).location;
      (*scope_raw).return_type = parent.return_type;
      (*scope_raw).vararg_pack = parent.vararg_pack;

      let parent_raw = parent.as_ref() as *const Scope as *mut Scope;
      (*parent_raw).children.push(scope_raw);
    }

    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_scopes
          .get_or_insert(node as *const AstNode) = scope_raw;
      }
    }

    scope
  }
}
