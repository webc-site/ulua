//! @interface-stub
use alloc::sync::Arc;

use ulua_ast::records::ast_type::AstType;

use crate::{
  records::{module::Module, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn resolve_type(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
    let ty = self.resolve_type_worker(scope, annotation);

    unsafe {
      let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
      *(*module)
        .ast_resolved_types
        .get_or_insert(annotation as *const AstType) = ty;
    }

    ty
  }
}
